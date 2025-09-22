# Tasks: Gemini OCR Integration

**Input**: Design documents from `/specs/006-implement-gemini-ocr/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   ✓ Loaded implementation plan successfully
   ✓ Extracted: Rust/Axum, reqwest, SQLx, PostgreSQL
2. Load optional design documents:
   ✓ data-model.md: GeminiOCRRequest, GeminiOCRResponse, BillData entities
   ✓ contracts/: /api/ocr endpoint enhancement contract
   ✓ research.md: Direct Gemini API calls via reqwest HTTP client
3. Generate tasks by category:
   ✓ Setup: Rust dependencies, environment configuration
   ✓ Core: Gemini service, OCR models, API handler enhancement
   ✓ Integration: Environment loading, error handling
   ✓ Polish: validation, logging (no unit tests per constitution)
4. Apply task rules:
   ✓ Different files = mark [P] for parallel
   ✓ Same file = sequential (no [P])
   ✓ Implementation-first (skip TDD per user request)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Web app**: `backend/src/`, `frontend/src/`
- Based on plan.md structure showing web application backend enhancement

## Phase 3.1: Setup
- [x] T001 Add reqwest and base64 dependencies to backend/Cargo.toml for Gemini API integration
- [x] T002 [P] Add required environment variables to backend/.env.example (GEMINI_API_KEY, GEMINI_MODEL, etc.)
- [x] T003 [P] Update backend/src/config/mod.rs to load Gemini configuration from environment

## Phase 3.2: Core Models (Implementation-First)
- [x] T004 [P] Create backend/src/models/gemini_request.rs with GeminiOCRRequest, ImageData, ProcessingOptions structs
- [x] T005 [P] Create backend/src/models/gemini_response.rs with GeminiOCRResponse, BillExtractionResult, ProcessingSummary structs
- [x] T006 [P] Create backend/src/models/ocr_error.rs with ProcessingError enum and error types
- [x] T007 [P] Create backend/src/models/bill_data.rs with BillData struct mapping to Bills table schema

## Phase 3.3: Gemini Service Implementation
- [x] T008 Create backend/src/services/gemini_service.rs with GeminiService struct and HTTP client setup
- [x] T009 Implement prepare_gemini_request method in backend/src/services/gemini_service.rs for JSON request construction
- [x] T010 Implement call_gemini_api method in backend/src/services/gemini_service.rs for direct HTTP calls to generativelanguage.googleapis.com
- [x] T011 Implement parse_gemini_response method in backend/src/services/gemini_service.rs for structured bill data extraction
- [ ] T012 Implement process_images method in backend/src/services/gemini_service.rs for sequential image processing

## Phase 3.4: API Handler Enhancement
- [ ] T013 Update backend/src/api/ocr_handler.rs to import and initialize GeminiService
- [ ] T014 Modify process_ocr_request function in backend/src/api/ocr_handler.rs to use Gemini instead of existing OCR
- [ ] T015 Add image validation logic in backend/src/api/ocr_handler.rs (MIME type, size limits)
- [ ] T016 Update response formatting in backend/src/api/ocr_handler.rs to return structured BillData JSON

## Phase 3.5: Integration & Configuration
- [ ] T017 Update backend/src/main.rs to register enhanced OCR routes with Gemini integration
- [ ] T018 [P] Add Gemini client configuration initialization to backend/src/config/mod.rs
- [ ] T019 [P] Implement comprehensive error handling in backend/src/services/gemini_service.rs for API failures
- [ ] T020 [P] Add structured logging for Gemini API calls in backend/src/services/gemini_service.rs

## Phase 3.6: Validation & Testing Setup
- [ ] T021 [P] Create manual test script for Vietnamese bill image processing following quickstart.md scenarios
- [ ] T022 [P] Add input validation for multipart form-data image uploads in backend/src/api/ocr_handler.rs
- [ ] T023 [P] Implement timeout handling and graceful degradation in backend/src/services/gemini_service.rs
- [ ] T024 [P] Add environment variable validation on service startup in backend/src/config/mod.rs

## Dependencies
- Setup (T001-T003) before models (T004-T007)
- Models (T004-T007) before service (T008-T012)
- Service (T008-T012) before handler (T013-T016)
- T008 blocks T009, T010, T011, T012
- T013 blocks T014, T015, T016
- Integration (T017-T020) requires completed handler (T016)
- Validation (T021-T024) can start after T008 (service exists)

## Parallel Example
```
# Launch T004-T007 together (different model files):
Task: "Create backend/src/models/gemini_request.rs with GeminiOCRRequest, ImageData, ProcessingOptions structs"
Task: "Create backend/src/models/gemini_response.rs with GeminiOCRResponse, BillExtractionResult, ProcessingSummary structs"
Task: "Create backend/src/models/ocr_error.rs with ProcessingError enum and error types"
Task: "Create backend/src/models/bill_data.rs with BillData struct mapping to Bills table schema"
```

```
# Launch T018-T020 together (different files):
Task: "Add Gemini client configuration initialization to backend/src/config/mod.rs"
Task: "Implement comprehensive error handling in backend/src/services/gemini_service.rs for API failures"
Task: "Add structured logging for Gemini API calls in backend/src/services/gemini_service.rs"
```

```
# Launch T021-T024 together (validation tasks):
Task: "Create manual test script for Vietnamese bill image processing following quickstart.md scenarios"
Task: "Add input validation for multipart form-data image uploads in backend/src/api/ocr_handler.rs"
Task: "Implement timeout handling and graceful degradation in backend/src/services/gemini_service.rs"
Task: "Add environment variable validation on service startup in backend/src/config/mod.rs"
```

## Implementation Notes
- **Skip TDD**: Implementation-first approach per user request and constitutional requirements
- **Sequential Processing**: Images processed one at a time to respect API limits
- **Environment Config**: All Gemini settings loaded from .env variables
- **Error Handling**: Graceful degradation when API fails
- **Vietnamese Support**: UTF-8 encoding preserved throughout processing chain
- **Database Integration**: BillData struct matches existing Bills table schema exactly

## Task Generation Rules Applied

1. **From Contracts**:
   - /api/ocr endpoint → handler enhancement tasks (T013-T016)

2. **From Data Model**:
   - GeminiOCRRequest → model task (T004)
   - GeminiOCRResponse → model task (T005)
   - ProcessingError → model task (T006)
   - BillData → model task (T007)

3. **From Research**:
   - Direct HTTP calls → service implementation (T008-T012)
   - Environment configuration → config tasks (T002, T003, T018, T024)

4. **From Quickstart**:
   - Test scenarios → manual validation (T021)
   - Setup steps → environment configuration (T002)

## Validation Checklist
*GATE: Checked by main() before returning*

- [x] All contracts have corresponding implementation tasks
- [x] All entities have model tasks
- [x] Implementation-first approach (skipped TDD per request)
- [x] Parallel tasks truly independent (different files)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Dependencies properly sequenced
- [x] Constitutional compliance (no unit test requirements)