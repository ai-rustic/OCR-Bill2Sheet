from __future__ import annotations


from datetime import datetime
from decimal import Decimal
from io import BytesIO

from fastapi import APIRouter, Query, Response, status
from fastapi.responses import JSONResponse, StreamingResponse
from sqlalchemy import func, select
from sqlalchemy.exc import SQLAlchemyError

from openpyxl import Workbook

from app.database import DatabaseSessionDep
from models import Bill
from schemas import ApiResponse, BillCreate, BillRead, BillUpdate

router = APIRouter(prefix="/api/bills", tags=["bills"])


def _bill_to_schema(bill: Bill) -> BillRead:
    return BillRead.model_validate(bill, from_attributes=True)


def _error_response(message: str, status_code: int) -> JSONResponse:
    payload = ApiResponse[None].error_response(message)
    return JSONResponse(status_code=status_code, content=payload.model_dump())


@router.get("", response_model=ApiResponse[list[BillRead]])
async def list_bills(
    session: DatabaseSessionDep,
    page: int = Query(1, ge=1),
    limit: int = Query(10, ge=1, le=100),
) -> ApiResponse[list[BillRead]] | JSONResponse:
    offset = (page - 1) * limit

    try:
        result = await session.execute(
            select(Bill).order_by(Bill.id.asc()).offset(offset).limit(limit)
        )
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to fetch bills: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    bills = result.scalars().all()
    data = [_bill_to_schema(bill) for bill in bills]
    return ApiResponse.success_response(data)


@router.get("/export", response_class=StreamingResponse)
async def export_bills(
    session: DatabaseSessionDep,
    format: str = Query("xlsx"),
) -> Response:
    export_format = format.lower()
    if export_format != "xlsx":
        return _error_response(
            "Only 'xlsx' export format is supported",
            status.HTTP_400_BAD_REQUEST,
        )

    try:
        result = await session.execute(select(Bill).order_by(Bill.id.asc()))
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to export bills: {exc}",
            status.HTTP_500_INTERNAL_SERVER_ERROR,
        )

    bills = result.scalars().all()

    workbook = Workbook()
    sheet = workbook.active
    sheet.title = "Bills"

    columns = [
        ("ID", "id"),
        ("Form No", "form_no"),
        ("Serial No", "serial_no"),
        ("Invoice No", "invoice_no"),
        ("Issued Date", "issued_date"),
        ("Seller Name", "seller_name"),
        ("Seller Tax Code", "seller_tax_code"),
        ("Item Name", "item_name"),
        ("Unit", "unit"),
        ("Quantity", "quantity"),
        ("Unit Price", "unit_price"),
        ("Total Amount", "total_amount"),
        ("VAT Rate", "vat_rate"),
        ("VAT Amount", "vat_amount"),
    ]

    sheet.append([header for header, _ in columns])

    for bill in bills:
        row = []
        for _, attr in columns:
            value = getattr(bill, attr)
            if isinstance(value, Decimal):
                value = float(value)
            row.append(value)
        sheet.append(row)

    buffer = BytesIO()
    workbook.save(buffer)
    buffer.seek(0)

    timestamp = datetime.utcnow().strftime("%Y%m%d%H%M%S")
    filename = f"bills-{timestamp}.xlsx"

    headers = {
        "Content-Disposition": f"attachment; filename={filename}",
    }

    return StreamingResponse(
        buffer,
        media_type="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        headers=headers,
    )


@router.get("/{bill_id}", response_model=ApiResponse[BillRead])
async def get_bill(
    bill_id: int,
    session: DatabaseSessionDep,
) -> ApiResponse[BillRead] | JSONResponse:
    try:
        result = await session.execute(select(Bill).where(Bill.id == bill_id))
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to fetch bill: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    bill = result.scalar_one_or_none()
    if bill is None:
        return _error_response(
            f"Bill with ID {bill_id} not found", status.HTTP_404_NOT_FOUND
        )

    return ApiResponse.success_response(_bill_to_schema(bill))


@router.post("", response_model=ApiResponse[BillRead], status_code=status.HTTP_201_CREATED)
async def create_bill(
    payload: BillCreate,
    session: DatabaseSessionDep,
) -> ApiResponse[BillRead] | JSONResponse:
    bill = Bill(**payload.model_dump(exclude_unset=True))
    session.add(bill)

    try:
        await session.commit()
        await session.refresh(bill)
    except SQLAlchemyError as exc:
        await session.rollback()
        return _error_response(
            f"Failed to create bill: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    return ApiResponse.success_response(_bill_to_schema(bill))


@router.put("/{bill_id}", response_model=ApiResponse[BillRead])
async def update_bill(
    bill_id: int,
    payload: BillUpdate,
    session: DatabaseSessionDep,
) -> ApiResponse[BillRead] | JSONResponse:
    try:
        result = await session.execute(select(Bill).where(Bill.id == bill_id))
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to fetch bill: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    bill = result.scalar_one_or_none()
    if bill is None:
        return _error_response(
            f"Bill with ID {bill_id} not found", status.HTTP_404_NOT_FOUND
        )

    update_data = payload.model_dump(exclude_unset=True)
    for field_name, value in update_data.items():
        setattr(bill, field_name, value)

    try:
        await session.commit()
        await session.refresh(bill)
    except SQLAlchemyError as exc:
        await session.rollback()
        return _error_response(
            f"Failed to update bill: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    return ApiResponse.success_response(_bill_to_schema(bill))


@router.delete("/{bill_id}", response_model=ApiResponse[str])
async def delete_bill(
    bill_id: int,
    session: DatabaseSessionDep,
) -> ApiResponse[str] | JSONResponse:
    try:
        result = await session.execute(select(Bill).where(Bill.id == bill_id))
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to fetch bill: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    bill = result.scalar_one_or_none()
    if bill is None:
        return _error_response(
            f"Bill with ID {bill_id} not found", status.HTTP_404_NOT_FOUND
        )

    try:
        await session.delete(bill)
        await session.commit()
    except SQLAlchemyError as exc:
        await session.rollback()
        return _error_response(
            f"Failed to delete bill: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    message = f"Bill with ID {bill_id} deleted successfully"
    return ApiResponse.success_response(message)


@router.get("/search", response_model=ApiResponse[list[BillRead]])
async def search_bills(
    session: DatabaseSessionDep,
    query: str | None = Query(None, alias="q"),
    invoice: str | None = Query(None, alias="invoice"),
) -> ApiResponse[list[BillRead]] | JSONResponse:
    raw_term = query or invoice

    if not raw_term or not raw_term.strip():
        return _error_response(
            "Search query parameter 'q' or 'invoice' is required",
            status.HTTP_400_BAD_REQUEST,
        )

    search_term = raw_term.strip()

    try:
        result = await session.execute(
            select(Bill)
            .where(Bill.invoice_no.ilike(f"%{search_term}%"))
            .order_by(Bill.issued_date.desc())
        )
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to search bills: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    bills = result.scalars().all()
    data = [_bill_to_schema(bill) for bill in bills]
    return ApiResponse.success_response(data)


@router.get("/count", response_model=ApiResponse[int])
async def count_bills(session: DatabaseSessionDep) -> ApiResponse[int] | JSONResponse:
    try:
        result = await session.execute(select(func.count()).select_from(Bill))
    except SQLAlchemyError as exc:
        return _error_response(
            f"Failed to get bills count: {exc}", status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    count = result.scalar_one()
    return ApiResponse.success_response(int(count))

