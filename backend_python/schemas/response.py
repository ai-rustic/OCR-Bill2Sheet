from typing import Generic, Optional, TypeVar

from pydantic import BaseModel, ConfigDict

T = TypeVar("T")


class ApiResponse(BaseModel, Generic[T]):
    success: bool
    data: Optional[T] = None
    error: Optional[str] = None

    model_config = ConfigDict(extra="forbid")

    @classmethod
    def success_response(cls, data: T) -> "ApiResponse[T]":
        return cls(success=True, data=data)

    @classmethod
    def error_response(cls, message: str) -> "ApiResponse[T]":
        return cls(success=False, error=message)
