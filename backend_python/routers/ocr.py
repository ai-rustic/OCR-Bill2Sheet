from __future__ import annotations

import json
from typing import Any, AsyncIterator, NamedTuple

import httpx
from fastapi import APIRouter, File, HTTPException, UploadFile, status
from pydantic import ValidationError
from sqlalchemy.exc import SQLAlchemyError
from sse_starlette.sse import EventSourceResponse

from app.database import DatabaseSessionDep
from app.gemini import GeminiError, extract_bill_items
from models import Bill
from schemas import BillCreate, BillRead


class _BufferedImage(NamedTuple):
    filename: str
    mime_type: str
    content: bytes


router = APIRouter(prefix="/api/ocr", tags=["ocr"])


def _sse_event(event: str, data: dict[str, Any]) -> dict[str, str]:
    return {"event": event, "data": json.dumps(data, ensure_ascii=False)}


@router.post("", response_class=EventSourceResponse, status_code=status.HTTP_202_ACCEPTED)
async def process_ocr_images(
    session: DatabaseSessionDep,
    images: list[UploadFile] = File(..., description="List of bill images to process"),
) -> EventSourceResponse:
    if not images:
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail="No images provided")

    buffered_files: list[_BufferedImage] = []
    for upload in images:
        filename = upload.filename or "unknown"
        mime_type = upload.content_type or "image/jpeg"
        try:
            content = await upload.read()
        finally:
            await upload.close()

        buffered_files.append(
            _BufferedImage(
                filename=filename,
                mime_type=mime_type,
                content=content,
            )
        )

    async def event_stream() -> AsyncIterator[dict[str, str]]:
        async with httpx.AsyncClient(timeout=httpx.Timeout(120.0)) as client:
            for index, buffered in enumerate(buffered_files, start=1):
                yield _sse_event(
                    "image_started",
                    {"image_index": index, "filename": buffered.filename},
                )

                try:
                    if not buffered.content:
                        raise ValueError("Uploaded image is empty")

                    yield _sse_event(
                        "image_processing",
                        {"image_index": index, "filename": buffered.filename},
                    )

                    raw_items = await extract_bill_items(client, buffered.content, buffered.mime_type)

                    bill_records: list[Bill] = []
                    for item in raw_items:
                        if not isinstance(item, dict):
                            raise GeminiError("Gemini response item is not an object")
                        try:
                            payload = BillCreate.model_validate(item)
                        except ValidationError as exc:
                            raise GeminiError(
                                f"Gemini response item validation failed: {exc.errors()}"
                            ) from exc

                        bill = Bill(**payload.model_dump(exclude_unset=True))
                        session.add(bill)
                        bill_records.append(bill)

                    items_payload: list[dict[str, Any]] = []
                    if bill_records:
                        try:
                            await session.commit()
                        except SQLAlchemyError as exc:
                            await session.rollback()
                            raise GeminiError(f"Failed to save bills to database: {exc}") from exc

                        for bill in bill_records:
                            await session.refresh(bill)
                            bill_schema = BillRead.model_validate(bill, from_attributes=True)
                            items_payload.append(bill_schema.model_dump(mode="json"))
                    else:
                        await session.rollback()

                    yield _sse_event(
                        "image_completed",
                        {
                            "image_index": index,
                            "filename": buffered.filename,
                            "items": items_payload,
                        },
                    )

                except (GeminiError, ValueError) as exc:
                    await session.rollback()
                    yield _sse_event(
                        "image_failed",
                        {
                            "image_index": index,
                            "filename": buffered.filename,
                            "message": str(exc),
                        },
                    )
                except Exception as exc:  # pragma: no cover - safeguard
                    await session.rollback()
                    yield _sse_event(
                        "image_failed",
                        {
                            "image_index": index,
                            "filename": buffered.filename,
                            "message": str(exc),
                        },
                    )

            yield _sse_event("finished", {"processed": len(buffered_files)})

    return EventSourceResponse(event_stream())
