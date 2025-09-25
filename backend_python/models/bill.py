from datetime import date
from decimal import Decimal

from sqlalchemy import Date, Numeric, Text
from sqlalchemy.orm import Mapped, mapped_column

from .base import Base


class Bill(Base):
    __tablename__ = "bills"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    form_no: Mapped[str | None] = mapped_column(Text, nullable=True)
    serial_no: Mapped[str | None] = mapped_column(Text, nullable=True)
    invoice_no: Mapped[str | None] = mapped_column(Text, nullable=True)
    issued_date: Mapped[date | None] = mapped_column(Date, nullable=True)
    seller_name: Mapped[str | None] = mapped_column(Text, nullable=True)
    seller_tax_code: Mapped[str | None] = mapped_column(Text, nullable=True)
    item_name: Mapped[str | None] = mapped_column(Text, nullable=True)
    unit: Mapped[str | None] = mapped_column(Text, nullable=True)
    quantity: Mapped[Decimal | None] = mapped_column(Numeric(18, 2), nullable=True)
    unit_price: Mapped[Decimal | None] = mapped_column(Numeric(18, 2), nullable=True)
    total_amount: Mapped[Decimal | None] = mapped_column(Numeric(18, 2), nullable=True)
    vat_rate: Mapped[Decimal | None] = mapped_column(Numeric(5, 2), nullable=True)
    vat_amount: Mapped[Decimal | None] = mapped_column(Numeric(18, 2), nullable=True)
