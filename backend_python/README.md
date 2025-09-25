# OCR Bill2Sheet FastAPI backend

## Setup
- Create a virtual environment: python -m venv .venv
- Activate it (Linux/macOS): source .venv/bin/activate
- Activate it (Windows): .venv\Scripts\activate
- Install dependencies: pip install -r requirements.txt
- Copy .env.example to .env and adjust the PostgreSQL credentials if needed

## Run locally
Start the development server with:

    uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

Open http://127.0.0.1:8000/docs to explore the interactive API docs. The GET /health/db route validates database connectivity using the configured PostgreSQL DSN, and any route can reuse the shared database dependency defined in app/database.py.
## Notable endpoints
- `GET /api/bills/export?format=xlsx` downloads all bills as an Excel workbook (the `format` query currently supports only `xlsx`).

