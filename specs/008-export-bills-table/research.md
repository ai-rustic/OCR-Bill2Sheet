# Research: Export Bills Table Technical Decisions

**Phase**: 0 - Technical Research
**Date**: 2025-09-24
**Feature**: Bills table export as CSV/XLSX

## CSV Generation with UTF-8 BOM Support

### Decision: `csv` crate (v1.3+) + `unicode-bom` crate (v2.0+)
**Rationale**:
- Industry standard CSV library in Rust ecosystem (by BurntSushi)
- Native UTF-8 support for Vietnamese text through Rust's String type
- High performance with streaming capabilities
- Extensive documentation and active maintenance
- BOM support via separate lightweight crate ensures Excel compatibility

**Alternatives considered**:
- `qsv`: Rejected - designed as CLI tool, too heavy for library use
- Manual CSV writing: Rejected - error-prone for escaping and formatting
- `csv-core`: Rejected - too low-level, unnecessary complexity

**Implementation approach**: Write UTF-8 BOM header before CSV content for Excel compatibility with Vietnamese text.

## XLSX Generation with UTF-8 Encoding

### Decision: `rust_xlsxwriter` crate (v0.78+)
**Rationale**:
- Created by original libxlsxwriter author (John McNamara)
- Native Rust implementation, 3.81x faster than Python equivalent
- Full Excel compatibility with comprehensive UTF-8 support
- Rich feature set including formatting, auto-fit columns
- Active development and excellent documentation

**Alternatives considered**:
- `xlsxwriter` (Rust bindings): Rejected - requires C library dependency, less performant
- `calamine`: Rejected - read-only library, no write capabilities
- `umya-spreadsheet`: Rejected - less mature, limited documentation
- `simple_excel_writer`: Rejected - limited feature set, poor UTF-8 handling

**Implementation approach**: Use native worksheet creation with proper column formatting and auto-sizing.

## Memory-Efficient Streaming for Large Datasets

### Decision: `tokio-stream` (v0.1+) + `futures-util` (v0.3+)
**Rationale**:
- Native async streaming support integrated with Axum
- Constant memory usage regardless of dataset size
- Built on proven Tokio async runtime
- Seamless integration with SQLx for database streaming
- Client can start downloading immediately

**Alternatives considered**:
- In-memory generation: Rejected - memory usage scales with dataset size
- File-based temporary storage: Rejected - unnecessary disk I/O complexity
- Manual chunking: Rejected - `tokio-stream` provides robust abstractions

**Implementation approach**: Stream database results in chunks of 1000-5000 records, converting each chunk to format incrementally.

## File Download Response Patterns in Axum

### Decision: Header tuples with proper MIME types and cache control
**Rationale**:
- Standard HTTP headers ensure proper browser file handling
- Content-Disposition attachment forces download vs inline display
- Charset specification critical for Vietnamese text rendering
- Cache control prevents stale file downloads
- Error handling with appropriate HTTP status codes

**Alternatives considered**:
- Basic Response::builder(): Rejected - missing crucial headers for file downloads
- Generic binary response: Rejected - poor user experience, no filename
- Inline content disposition: Rejected - requirement is file download not display

**Implementation approach**: Use comprehensive header set with meaningful timestamped filenames and proper error responses.

## Performance Considerations

### Large Dataset Handling
- **Chunk size**: 1000-5000 records optimal balance between memory and network efficiency
- **Streaming threshold**: Use streaming for datasets >10MB
- **Memory usage**: Constant O(chunk_size) regardless of total dataset size
- **Download experience**: Client receives data immediately, can show progress

### Error Handling Strategy
- Graceful database connection failures
- Partial data corruption handling
- Client disconnection resilience
- Comprehensive error logging for debugging

## Dependencies Summary

```toml
[dependencies]
# CSV generation
csv = "1.3"
unicode-bom = "2.0"

# XLSX generation
rust_xlsxwriter = "0.78"

# Async streaming
tokio-stream = "0.1"
futures-util = "0.3"

# Existing dependencies (already in project)
axum = "0.8.4"
sqlx = "0.8.6"
tokio = "1.47.1"
```

## Technical Unknowns Resolved

✅ **CSV UTF-8 BOM Support**: Use `unicode-bom` crate to prefix CSV content
✅ **XLSX UTF-8 Encoding**: `rust_xlsxwriter` handles UTF-8 natively
✅ **Memory-Efficient Streaming**: `tokio-stream` with database chunking
✅ **Axum File Responses**: Header-based approach with proper MIME types
✅ **Vietnamese Text Compatibility**: Both libraries support full Unicode/UTF-8

All technical research complete and ready for Phase 1 design.