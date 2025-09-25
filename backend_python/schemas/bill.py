from datetime import date
from decimal import Decimal
from typing import Optional

from pydantic import BaseModel, ConfigDict


class BillBase(BaseModel):
    form_no: Optional[str] = None
    serial_no: Optional[str] = None
    invoice_no: Optional[str] = None
    issued_date: Optional[date] = None
    seller_name: Optional[str] = None
    seller_tax_code: Optional[str] = None
    item_name: Optional[str] = None
    unit: Optional[str] = None
    quantity: Optional[Decimal] = None
    unit_price: Optional[Decimal] = None
    total_amount: Optional[Decimal] = None
    vat_rate: Optional[Decimal] = None
    vat_amount: Optional[Decimal] = None

    model_config = ConfigDict(extra="forbid")


class BillCreate(BillBase):
    pass


class BillUpdate(BillBase):
    pass


class BillRead(BillBase):
    id: int

    model_config = ConfigDict(from_attributes=True)
