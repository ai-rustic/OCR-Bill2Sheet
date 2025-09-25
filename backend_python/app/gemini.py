from __future__ import annotations

import asyncio
import base64
import json
from collections.abc import Iterator
from itertools import cycle
from typing import Any

import httpx

from .config import get_settings


class GeminiError(RuntimeError):
    """Raised when the Gemini API request or response fails."""

# Rotate through configured Gemini API keys safely across tasks
_API_KEY_ITERATOR: Iterator[str] | None = None
_API_KEY_CACHE: tuple[str, ...] | None = None
_API_KEY_LOCK = asyncio.Lock()

_PROMPT = (
    "You are an OCR assistant that extracts structured bill data from an image. "
    "Return a JSON object with two keys: 'invoice' and 'items'. "
    "'invoice' describes invoice-level metadata with these keys: form_no, serial_no, invoice_no, issued_date (YYYY-MM-DD), "
    "seller_name, seller_tax_code. "
    "'items' is an array where every element contains: item_name, unit, quantity, unit_price, total_amount, vat_rate, vat_amount. "
    "Use null for any value that cannot be determined. Respond with JSON only (no markdown, no code fences)."
)


_RESPONSE_SCHEMA = {
    "type": "object",
    "properties": {
        "invoice": {
            "type": "object",
            "properties": {
                "form_no": {"type": "string"},
                "serial_no": {"type": "string"},
                "invoice_no": {"type": "string"},
                "issued_date": {"type": "string", "format": "date"},
                "seller_name": {"type": "string"},
                "seller_tax_code": {"type": "string"},
            },
            "required": [
                "form_no",
                "serial_no",
                "invoice_no",
                "issued_date",
                "seller_name",
                "seller_tax_code",
            ],
        },
        "items": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "item_name": {"type": "string"},
                    "unit": {"type": "string"},
                    "quantity": {"type": "number"},
                    "unit_price": {"type": "number"},
                    "total_amount": {"type": "number"},
                    "vat_rate": {"type": "number"},
                    "vat_amount": {"type": "number"},
                },
                "required": [
                    "item_name",
                    "unit",
                    "quantity",
                    "unit_price",
                    "total_amount",
                    "vat_rate",
                    "vat_amount",
                ],
            },
        },
    },
    "required": ["invoice", "items"],
}


def _strip_code_fences(raw_text: str) -> str:
    text = raw_text.strip()
    if text.startswith("```") and text.endswith("```"):
        lines = text.splitlines()
        if len(lines) >= 2:
            return "\n".join(lines[1:-1]).strip()
    return text


def _decode_candidates(payload: dict[str, Any]) -> str:
    candidates = payload.get("candidates") or []
    for candidate in candidates:
        content = candidate.get("content") or {}
        parts = content.get("parts") or []
        for part in parts:
            text = part.get("text")
            if text:
                return _strip_code_fences(text)
    raise GeminiError("Gemini response did not contain text content")


def _prepare_image_part(image_bytes: bytes, mime_type: str) -> dict[str, Any]:
    encoded = base64.b64encode(image_bytes).decode("utf-8")
    return {"inline_data": {"mime_type": mime_type, "data": encoded}}

async def _get_next_gemini_api_key(settings) -> str:
    keys = settings.resolved_gemini_api_keys
    if not keys:
        raise GeminiError("Gemini API key is not configured")

    key_tuple = tuple(keys)
    global _API_KEY_ITERATOR, _API_KEY_CACHE
    async with _API_KEY_LOCK:
        if _API_KEY_ITERATOR is None or _API_KEY_CACHE != key_tuple:
            _API_KEY_ITERATOR = cycle(key_tuple)
            _API_KEY_CACHE = key_tuple
        return next(_API_KEY_ITERATOR)


async def extract_bill_items(
    client: httpx.AsyncClient,
    image_bytes: bytes,
    mime_type: str,
) -> dict[str, Any]:
    settings = get_settings()

    api_key = await _get_next_gemini_api_key(settings)

    url = f"{settings.gemini_api_url}/{settings.gemini_model}:generateContent"

    payload: dict[str, Any] = {
        "contents": [
            {
                "role": "user",
                "parts": [
                    {"text": _PROMPT},
                    _prepare_image_part(image_bytes, mime_type),
                ],
            }
        ],
        "generationConfig": {
            "responseMimeType": "application/json",
            "responseSchema": _RESPONSE_SCHEMA,
        },
    }

    response = await client.post(url, params={"key": api_key}, json=payload)

    if response.status_code >= 400:
        detail = response.text
        raise GeminiError(f"Gemini request failed ({response.status_code}): {detail}")

    data = response.json()
    raw_text = _decode_candidates(data)

    try:
        parsed = json.loads(raw_text)
    except json.JSONDecodeError as exc:
        raise GeminiError(f"Gemini response is not valid JSON: {exc}") from exc

    if not isinstance(parsed, dict):
        raise GeminiError("Gemini response JSON must be an object")

    invoice = parsed.get("invoice")
    items = parsed.get("items")
    if not isinstance(invoice, dict):
        raise GeminiError("Gemini response is missing 'invoice' object")
    if not isinstance(items, list):
        raise GeminiError("Gemini response 'items' must be a list")

    return parsed
