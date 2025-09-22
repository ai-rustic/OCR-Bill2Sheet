# Tasks: Modify /api/ocr endpoint to response in SSE (Server-Sent Events)

**Input**: Design documents from `/specs/005-modify-api-ocr/`
**Prerequisites**: plan.md, data-model.md, contracts/sse-api-contract.md, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   ✅ Tech stack: Rust 1.75+, Axum 0.8.4, SSE implementation
   ✅ Structure: Web app (backend/src/ modification)
2. Load optional design documents:
   ✅ data-model.md: ProcessingEvent enum, ValidationErrorCode, ProcessingErrorType
   ✅ contracts/: POST /api/ocr SSE endpoint specification
   ✅ quickstart.md: Step-by-step implementation guide
3. Generate tasks by category:
   ✅ Setup: dependencies, models
   ✅ Core: SSE event types, handler modification
   ✅ Integration: state management, routing
   ✅ Verification: contract compliance testing
4. Apply task rules:
   ✅ Different files = mark [P] for parallel
   ✅ Same file = sequential (no [P])
   ✅ Implementation-first approach (TDD skipped per request)
5. Number tasks sequentially (T001-T012)
6. Generate dependency graph
7. Create parallel execution examples
8. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Web app**: `backend/src/`, `backend/Cargo.toml`
- Paths assume existing backend structure from OCR_Bill2Sheet project

## Phase 3.1: Setup
- [x] T001 Add SSE dependencies to backend/Cargo.toml (futures-util, tokio-stream, async-stream, uuid)
- [x] T002 [P] Create SSE event models in backend/src/models/sse_events.rs
- [x] T003 [P] Update models module exports in backend/src/models/mod.rs

## Phase 3.2: Core Implementation
- [x] T004 Create enhanced ImageFileInfo with SSE fields in backend/src/models/mod.rs
- [x] T005 Create SSE application state extension in backend/src/main.rs
- [x] T006 Replace OCR handler with SSE streaming in backend/src/api/ocr.rs
- [x] T007 Add event broadcasting setup to main function in backend/src/main.rs
- [x] T008 Update router configuration for SSE endpoint in backend/src/main.rs

## Phase 3.3: Integration & Verification
- [x] T009 Build project and resolve compilation errors (cargo build)
- [x] T010 [P] Test SSE contract compliance with cURL commands
- [x] T011 [P] Verify event sequence and format with sample uploads
- [x] T012 Run linting and type checking (cargo clippy)

## Dependencies
- T001 (dependencies) blocks T002-T003
- T002-T003 (models) block T004-T006
- T005 (state) blocks T007-T008
- T006 (handler) blocks T007-T008
- T001-T008 (implementation) blocks T009-T012

## Parallel Example
```bash
# Launch T002-T003 together after T001:
# Task: "Create SSE event models in backend/src/models/sse_events.rs"
# Task: "Update models module exports in backend/src/models/mod.rs"

# Launch T010-T011 together after implementation:
# Task: "Test SSE contract compliance with cURL commands"
# Task: "Verify event sequence and format with sample uploads"
```

## Task Details

### T001: Add SSE Dependencies
**File**: `backend/Cargo.toml`
**Action**: Add the following dependencies to the [dependencies] section:
```toml
futures-util = "0.3"
tokio-stream = { version = "0.1", features = ["sync"] }
async-stream = "0.3"
uuid = { version = "1.0", features = ["v4"] }
```
**Verification**: `cargo check` should pass

### T002: Create SSE Event Models [P]
**File**: `backend/src/models/sse_events.rs` (new file)
**Action**: Implement ProcessingEvent enum with 8 event types as specified in data-model.md:
- UploadStarted, ImageReceived, ImageValidationStart, ImageValidationSuccess
- ImageValidationError, AllImagesValidated, ProcessingComplete, ProcessingError
- Include ValidationErrorCode and ProcessingErrorType enums
**Dependencies**: chrono, serde, uuid imports
**Verification**: File compiles without errors

### T003: Update Models Module [P]
**File**: `backend/src/models/mod.rs`
**Action**: Add `pub mod sse_events;` to expose new event types
**Verification**: Models can be imported in other modules

### T004: Enhanced ImageFileInfo
**File**: `backend/src/models/mod.rs`
**Action**: Add SSE-specific fields to ImageFileInfo struct:
- `file_index: usize`
- `processed_at: chrono::DateTime<chrono::Utc>`
- `processing_duration_ms: u64`
**Verification**: Struct compiles and existing code still works

### T005: SSE Application State
**File**: `backend/src/main.rs`
**Action**: Add broadcast::Sender<ProcessingEvent> to app state
- Import necessary SSE modules
- Create (event_broadcaster, _) = broadcast::channel(1000)
- Add broadcaster to AppState struct
**Verification**: App state compiles with new field

### T006: Replace OCR Handler
**File**: `backend/src/api/ocr.rs`
**Action**: Replace entire handler with SSE streaming implementation:
- Change return type to Sse<impl Stream<Item = Result<Event, Infallible>>>
- Implement background processing with event emission
- Add session-based event streaming
- Follow quickstart.md implementation guide exactly
**Verification**: Handler compiles and returns SSE response

### T007: Event Broadcasting Setup
**File**: `backend/src/main.rs`
**Action**: Initialize event broadcaster in main function:
- Create broadcast channel before app state
- Pass broadcaster to AppState constructor
**Dependencies**: T005 (app state) must be complete
**Verification**: Application starts without errors

### T008: Router Configuration
**File**: `backend/src/main.rs`
**Action**: Update router to use new SSE handler:
- Change route handler from `upload_images` to `upload_images_sse`
- Ensure correct import path for new handler
**Dependencies**: T006 (handler) must be complete
**Verification**: Routes compile and server starts

### T009: Build Project
**File**: Repository root
**Action**: Run `cargo build` and resolve any compilation errors
- Fix import statements
- Resolve type mismatches
- Ensure all dependencies are properly linked
**Verification**: Clean build with no errors

### T010: Contract Compliance Testing [P]
**File**: Command line testing
**Action**: Test SSE endpoint against contract specification:
```bash
curl -X POST http://localhost:3000/api/ocr \
  -H "Accept: text/event-stream" \
  -H "Cache-Control: no-cache" \
  -F "images=@test1.jpg" \
  -F "images=@test2.png" \
  --no-buffer
```
**Verification**: Receives all 8 event types in correct sequence

### T011: Event Sequence Verification [P]
**File**: Command line testing
**Action**: Verify event format matches contract:
- Check event type names match specification
- Verify JSON data structure compliance
- Test error scenarios (invalid files, size limits)
- Confirm completion events close stream properly
**Verification**: Events match sse-api-contract.md exactly

### T012: Code Quality Check
**File**: Repository root
**Action**: Run final quality checks:
- `cargo clippy` for linting warnings
- `cargo test` if any existing tests
- Verify CLAUDE.md commands still work
**Verification**: All checks pass without warnings

## Notes
- TDD skipped per user request - implementation-first approach
- All SSE events must match contract specification exactly
- Maintain existing file upload validation logic
- No backward compatibility with JSON responses required
- [P] tasks can run in parallel when their files don't conflict

## Task Generation Rules Applied

1. **From Contracts**: POST /api/ocr → T006 (handler implementation), T010-T011 (testing)
2. **From Data Model**: ProcessingEvent → T002, ValidationErrorCode → T002, Enhanced ImageFileInfo → T004
3. **From Quickstart**: Step-by-step implementation → T001-T008 sequence
4. **Ordering**: Setup → Models → Core → Integration → Verification

## Validation Checklist
- [x] All contracts have corresponding implementation tasks
- [x] All entities have model tasks
- [x] Implementation-first approach (TDD skipped)
- [x] Parallel tasks are truly independent
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task