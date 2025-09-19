CREATE TABLE bills (
    id SERIAL PRIMARY KEY,
    form_no TEXT,
    serial_no TEXT,
    invoice_no TEXT,
    issued_date DATE,
    seller_name TEXT,
    seller_tax_code TEXT,
    item_name TEXT,
    unit TEXT,
    quantity NUMERIC(18,2),
    unit_price NUMERIC(18,2),
    total_amount NUMERIC(18,2),
    vat_rate NUMERIC(5,2),
    vat_amount NUMERIC(18,2)
);
