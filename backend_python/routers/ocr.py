from __future__ import annotations

import json
from datetime import datetime
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


_DATE_FORMATS = (
    "%Y-%m-%d",
    "%d/%m/%Y",
    "%d-%m-%Y",
    "%Y/%m/%d",
    "%m/%d/%Y",
    "%m-%d-%Y",
)


_INVOICE_FIELDS = (
    "form_no",
    "serial_no",
    "invoice_no",
    "issued_date",
    "seller_name",
    "seller_tax_code",
)


def _normalize_issued_date(target: dict[str, Any]) -> None:
    issued = target.get("issued_date")
    if isinstance(issued, str):
        value = issued.strip()
        if value:
            for fmt in _DATE_FORMATS:
                try:
                    target["issued_date"] = datetime.strptime(value, fmt).date().isoformat()
                    return
                except ValueError:
                    continue
            target["issued_date"] = None


def _normalize_invoice(invoice: dict[str, Any]) -> dict[str, Any]:
    normalized = {field: invoice.get(field) for field in _INVOICE_FIELDS}
    _normalize_issued_date(normalized)
    return normalized


def _merge_invoice_item(invoice: dict[str, Any], item: dict[str, Any]) -> dict[str, Any]:
    merged = {**invoice, **item}
    _normalize_issued_date(merged)
    return merged


router = APIRouter(prefix="/api/ocr", tags=["ocr"])


def _sse_event(event: str, data: dict[str, Any]) -> dict[str, str]:
    return {"event": event, "data": json.dumps(data, ensure_ascii=False)}


@router.post("", response_class=EventSourceResponse, status_code=status.HTTP_202_ACCEPTED)
async def process_ocr_images(
    session: DatabaseSessionDep,
    files: list[UploadFile] = File(..., description="List of bill images to process"),
) -> EventSourceResponse:
    if not files:
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail="No images provided")

    buffered_files: list[_BufferedImage] = []
    for upload in files:
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

                    raw_payload = await extract_bill_items(client, buffered.content, buffered.mime_type)

                    invoice_raw = raw_payload.get("invoice")
                    items_raw = raw_payload.get("items")
                    if not isinstance(invoice_raw, dict):
                        raise GeminiError("Gemini invoice payload is invalid")
                    if not isinstance(items_raw, list):
                        raise GeminiError("Gemini items payload must be a list")

                    invoice_data = _normalize_invoice(invoice_raw)
                    bill_records: list[Bill] = []

                    for item in items_raw:
                        if not isinstance(item, dict):
                            raise GeminiError("Gemini response item is not an object")
                        merged_item = _merge_invoice_item(invoice_data, item)
                        try:
                            payload = BillCreate.model_validate(merged_item)
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
                            "invoice": invoice_data,
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

            yield _sse_event(
                "finished",
                {"processed": len(buffered_files)},
            )

    return EventSourceResponse(event_stream())
