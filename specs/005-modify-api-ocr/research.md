# Research: SSE Implementation for /api/ocr Endpoint

## Overview
Research findings for implementing Server-Sent Events (SSE) in the existing Axum-based OCR endpoint to provide real-time streaming of image validation progress.

## Technical Decisions

### 1. SSE Framework Choice
**Decision**: Use Axum's built-in `axum::response::sse` module
**Rationale**:
- Native integration with existing Axum 0.8.4 framework
- Type-safe implementation with proper error handling
- Built-in keep-alive and connection management
- No additional dependencies beyond standard ecosystem tools

**Alternatives considered**:
- Custom SSE implementation - rejected due to complexity
- WebSocket upgrade - rejected as overkill for one-way streaming

### 2. Event Broadcasting Pattern
**Decision**: Use `tokio::sync::broadcast` channel for event distribution
**Rationale**:
- Multiple concurrent SSE connections can subscribe to same events
- Built-in backpressure handling for slow clients
- Efficient memory management with bounded channels
- Natural integration with Tokio async runtime

**Alternatives considered**:
- Direct streaming per request - rejected due to resource inefficiency
- Database-based event log - rejected as unnecessary persistence

### 3. Client Disconnection Handling
**Decision**: Implement Drop trait pattern with connection guards
**Rationale**:
- Automatic cleanup when SSE connection terminates
- No polling required for disconnection detection
- Rust's RAII ensures resource cleanup
- Simple to implement and maintain

**Alternatives considered**:
- Periodic ping/pong - rejected as it adds complexity
- Connection timeout only - rejected as insufficient for cleanup

### 4. Event Structure Format
**Decision**: JSON-formatted events with structured schema
**Rationale**:
- Consistent with existing API response format
- Type-safe serialization with serde
- Easy to extend with additional fields
- Compatible with frontend consumption

**Event Schema**:
```json
{
  "type": "image_validation_success",
  "timestamp": "2025-09-22T...",
  "data": {
    "file_index": 0,
    "file_name": "invoice.jpg",
    "size_bytes": 1024000,
    "format": "JPEG"
  }
}
```

### 5. Error Handling Strategy
**Decision**: Send error events through stream without breaking connection
**Rationale**:
- Maintains real-time feedback for partial failures
- Client can handle errors per-file basis
- Stream continues for remaining files
- Follows progressive enhancement principle

**Implementation**: Error events with `"type": "processing_error"` and continue processing

### 6. Performance Optimization
**Decision**: Bounded broadcast channels with 1000 event buffer
**Rationale**:
- Prevents memory exhaustion from slow clients
- Handles typical image processing workloads
- Reasonable latency vs memory tradeoff
- Can be adjusted via environment configuration

**Monitoring**: Implement lagged client detection and warnings

## Required Dependencies
Add to `backend/Cargo.toml`:
```toml
futures-util = "0.3"
tokio-stream = { version = "0.1", features = ["sync"] }
async-stream = "0.3"
uuid = { version = "1.0", features = ["v4"] }
```

## Architecture Changes

### 1. State Management
- Add `broadcast::Sender<ProcessingEvent>` to app state
- Initialize broadcaster in `main.rs` setup
- Share broadcaster across all request handlers

### 2. Endpoint Modification
- Replace JSON response with SSE stream response
- Maintain existing multipart upload handling
- Add event emission throughout validation process

### 3. Event Types
Eight event types as specified in requirements:
1. `upload_started` - Initial event with file count
2. `image_received` - Per-file reception confirmation
3. `image_validation_start` - Begin validation per file
4. `image_validation_success` - File passes validation
5. `image_validation_error` - File fails validation
6. `all_images_validated` - Batch validation complete
7. `processing_complete` - Final success event
8. `processing_error` - System-level errors

## Implementation Constraints

### 1. Memory Management
- Use bounded channels to prevent unbounded growth
- Implement client cleanup on disconnection
- Monitor for lagged clients and handle gracefully

### 2. Error Recovery
- Continue processing remaining files on individual failures
- Send error events but maintain stream connection
- Provide clear error messages with context

### 3. Connection Limits
- Implement reasonable concurrent connection limits
- Use tower middleware for connection management
- Monitor resource usage

## Integration Points

### 1. Existing Validation Logic
- Preserve all existing file size and count validations
- Maintain environment-based configuration
- Keep existing error types and handling

### 2. Response Format Change
- Remove JSON response entirely (no backward compatibility)
- SSE becomes the only response format
- Update content-type to `text/event-stream`

### 3. Client Expectations
- Immediate stream response on POST
- Progressive event updates during processing
- Final completion or error event to close stream

## Risk Mitigation

### 1. Resource Exhaustion
- Bounded channel buffers
- Connection count limits
- Client timeout handling

### 2. Network Reliability
- Built-in keep-alive messages
- No retry mechanism (per requirements)
- Client responsible for reconnection

### 3. Compatibility
- No backward compatibility required
- Frontend must be updated to consume SSE
- Clear migration path from JSON to SSE

## Success Criteria
1. Real-time streaming of validation events
2. Graceful handling of client disconnections
3. Memory-efficient operation under load
4. Maintains existing file upload functionality
5. Clear error reporting through event stream