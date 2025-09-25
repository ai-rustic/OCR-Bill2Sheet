from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy import text

from .database import DatabaseSessionDep
from routers import bills_router, ocr_router

app = FastAPI(title="OCR Bill2Sheet API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(bills_router)
app.include_router(ocr_router)


@app.get("/", tags=["root"])
async def read_root():
    return {"message": "Welcome to OCR Bill2Sheet API"}


@app.get("/health", tags=["health"])
async def health_check():
    return {"status": "ok"}


@app.get("/health/db", tags=["health"])
async def database_health(session: DatabaseSessionDep):
    await session.execute(text("SELECT 1"))
    return {"status": "ok"}
