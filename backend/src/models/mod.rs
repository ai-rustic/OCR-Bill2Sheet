pub mod bill;
pub mod bill_data;
pub mod gemini_request;
pub mod gemini_response;
pub mod image_info;
pub mod ocr_error;
pub mod sse_events;
pub mod validation_result;

pub use bill::{Bill, CreateBill};
pub use bill_data::{BillData as BillDataSchema, CreateBillData};
pub use gemini_request::{GeminiOCRRequest, ImageData, ProcessingOptions};
pub use gemini_response::{GeminiOCRResponse, BillExtractionResult, BillData, ConfidenceScores, ProcessingSummary, ProcessingError, ErrorType};
pub use image_info::{ImageFileInfo, ValidationStatus};
pub use ocr_error::{ProcessingError as OcrProcessingError, ErrorType as OcrErrorType, ProcessingErrorResponse};
pub use sse_events::{ProcessingEvent, ValidationErrorCode, ProcessingErrorType, SSEEventEnvelope, ProcessingSession, SessionStatus};
pub use validation_result::{ValidationResult, ValidationData};