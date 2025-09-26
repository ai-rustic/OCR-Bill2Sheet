from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
from sqlalchemy import text
import os
import pathlib

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

# Set up static file serving for frontend
FRONTEND_DIR = pathlib.Path(__file__).parent.parent.parent / "frontend" / "out"

# Include API routers FIRST (so they take precedence)
app.include_router(bills_router)
app.include_router(ocr_router)

# Mount static files (CSS, JS, images, etc.)
if FRONTEND_DIR.exists():
    app.mount("/_next", StaticFiles(directory=str(FRONTEND_DIR / "_next")), name="static")

# Serve specific static assets
@app.get("/favicon.ico")
async def favicon():
    favicon_path = FRONTEND_DIR / "favicon.ico"
    if favicon_path.exists():
        return FileResponse(favicon_path)
    return {"error": "Favicon not found"}

# Serve known static files
@app.get("/{file_name:path}")
async def static_files(file_name: str):
    """Serve static files like SVG, images, etc."""
    # Don't interfere with API routes
    if file_name.startswith("api/") or file_name in ["health", "docs", "redoc", "openapi.json"]:
        return {"error": "Not found"}

    file_path = FRONTEND_DIR / file_name
    if file_path.exists() and file_path.is_file():
        return FileResponse(file_path)

    # For frontend routing (bills/, 404/, etc.), serve index.html
    index_path = FRONTEND_DIR / "index.html"
    if index_path.exists():
        return FileResponse(index_path)
    return {"error": "File not found"}

@app.get("/")
async def read_root():
    """Serve the main frontend page"""
    index_path = FRONTEND_DIR / "index.html"
    if index_path.exists():
        return FileResponse(index_path)
    return {"message": "Welcome to OCR Bill2Sheet API - Frontend not found"}


@app.get("/health", tags=["health"])
async def health_check():
    return {"status": "ok"}


@app.get("/health/db", tags=["health"])
async def database_health(session: DatabaseSessionDep):
    await session.execute(text("SELECT 1"))
    return {"status": "ok"}
