# Tasks: OCR Image Upload API Endpoint

**Input**: Design documents from `/specs/004-add-post-api/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

**Context**: Skip TDD process - Implementation-first approach as per constitution

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → If not found: ERROR "No implementation plan found"
   → Extract: tech stack, libraries, structure
2. Load optional design documents:
   → data-model.md: Extract entities → model tasks
   → contracts/: Each file → contract test task
   → research.md: Extract decisions → setup tasks
3. Generate tasks by category:
   → Setup: project init, dependencies, linting
   → Core: models, services, API endpoints (Implementation-first)
   → Integration: Route integration, error handling
   → Validation: Integration tests, manual testing
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Implementation before tests (Non-TDD)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
Web application structure: `backend/src/`, based on plan.md analysis

## Phase 3.1: Setup and Dependencies
- [x] T001 Add required dependencies to backend/Cargo.toml (axum-typed-multipart, tempfile, image, infer, thiserror)
- [x] T002 [P] Create upload configuration module in backend/src/config/upload_config.rs
- [x] T003 [P] Create error types module in backend/src/errors/upload_error.rs
- [x] T004 Update backend/src/config/mod.rs to expose upload_config module
- [x] T005 Update backend/src/errors/mod.rs to expose upload_error module (if errors/mod.rs doesn't exist, create it)

## Phase 3.2: Core Data Models and Services
- [x] T006 [P] Create ImageFileInfo struct in backend/src/models/image_info.rs
- [x] T007 [P] Create ValidationResult struct in backend/src/models/validation_result.rs
- [x] T008 [P] Create image validation service in backend/src/services/image_validation.rs
- [x] T009 Update backend/src/models/mod.rs to expose image models
- [x] T010 Update backend/src/services/mod.rs to expose image_validation module

## Phase 3.3: API Implementation
- [x] T011 Create OCR upload handler in backend/src/api/ocr.rs with POST /api/ocr endpoint
- [x] T012 Update backend/src/api/mod.rs to expose ocr module (create if doesn't exist)
- [x] T013 Integrate OCR route in backend/src/main.rs with proper middleware and body limits
- [x] T014 Add environment variable loading and UploadConfig initialization in backend/src/main.rs

## Phase 3.4: Error Handling and Response Types
- [x] T015 [P] Implement IntoResponse for UploadError in backend/src/errors/upload_error.rs
- [x] T016 [P] Add JSON response serialization for ValidationResult in backend/src/models/validation_result.rs
- [x] T017 Add comprehensive error handling in OCR upload handler (backend/src/api/ocr.rs)

## Phase 3.5: Integration and Configuration
- [x] T018 Create .env template with MAX_FILE_SIZE_BYTES and MAX_IMAGE_COUNT in backend/.env.example
- [x] T019 Update backend/src/main.rs to load configuration from environment at startup
- [x] T020 Add request body size limits and timeout configuration to backend/src/main.rs

## Phase 3.6: Validation Testing (Non-TDD)
- [x] T021 [P] Create manual test script for valid single image upload
- [x] T022 [P] Create manual test script for multiple image uploads
- [x] T023 [P] Create manual test script for file size limit validation
- [x] T024 [P] Create manual test script for invalid image format validation
- [x] T025 [P] Create manual test script for image count limit validation

## Phase 3.7: Polish and Documentation
- [x] T026 [P] Add comprehensive error logging to image validation service
- [x] T027 Run cargo clippy and fix any warnings
- [x] T028 Run cargo test to ensure no regressions
- [x] T029 Update backend README with API endpoint documentation
- [x] T030 Verify quickstart.md instructions work with implementation

## Dependencies
### Critical Path
- T001 → T002, T003 (Dependencies before modules)
- T004, T005 → T006, T007, T008 (Module registration before implementations)
- T009, T010 → T011 (Models and services before API)
- T011 → T012 → T013 (API handler before routing)
- T013 → T014 (Route integration before main app config)
- T015, T016 → T017 (Response types before error handling)
- T018 → T019 (Environment template before loading)

### Independent Tasks (Parallel Opportunities)
- T002, T003 can run together (different files)
- T006, T007, T008 can run together (different files)
- T015, T016 can run together (different files)
- T021-T025 can run together (different test files)
- T026, T029 can run together (different concerns)

## Parallel Execution Examples

### Setup Phase (T002-T003)
```bash
# Can run simultaneously
Task: "Create upload configuration module in backend/src/config/upload_config.rs"
Task: "Create error types module in backend/src/errors/upload_error.rs"
```

### Models Phase (T006-T008)
```bash
# Can run simultaneously
Task: "Create ImageFileInfo struct in backend/src/models/image_info.rs"
Task: "Create ValidationResult struct in backend/src/models/validation_result.rs"
Task: "Create image validation service in backend/src/services/image_validation.rs"
```

### Response Handling (T015-T016)
```bash
# Can run simultaneously
Task: "Implement IntoResponse for UploadError in backend/src/errors/upload_error.rs"
Task: "Add JSON response serialization for ValidationResult in backend/src/models/validation_result.rs"
```

### Testing Phase (T021-T025)
```bash
# Can run simultaneously
Task: "Create manual test script for valid single image upload"
Task: "Create manual test script for multiple image uploads"
Task: "Create manual test script for file size limit validation"
Task: "Create manual test script for invalid image format validation"
Task: "Create manual test script for image count limit validation"
```

## Implementation Notes

### Key Technical Decisions (from research.md)
- Use Axum's native `Multipart` extractor with `axum-typed-multipart`
- Environment variable configuration with `dotenvy`
- Magic byte validation with `infer` crate + `image` crate parsing
- Custom error types with `thiserror` and `IntoResponse` implementation

### File Structure Created
```
backend/src/
├── config/
│   ├── mod.rs (updated in T004)
│   └── upload_config.rs (T002)
├── errors/
│   ├── mod.rs (created/updated in T005)
│   └── upload_error.rs (T003)
├── models/
│   ├── mod.rs (updated in T009)
│   ├── image_info.rs (T006)
│   └── validation_result.rs (T007)
├── services/
│   ├── mod.rs (updated in T010)
│   └── image_validation.rs (T008)
├── api/
│   ├── mod.rs (created/updated in T012)
│   └── ocr.rs (T011)
└── main.rs (updated in T013, T014, T019, T020)
```

### Configuration Files
```
backend/
├── .env.example (T018)
├── Cargo.toml (updated in T001)
└── README.md (updated in T029)
```

## Validation Checklist
*Verified during task generation*

- [x] All contracts have corresponding implementation tasks
- [x] All entities have model tasks (ImageFileInfo, ValidationResult, UploadConfig)
- [x] Implementation-first approach (no TDD as requested)
- [x] Parallel tasks truly independent (different files)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Dependencies properly mapped
- [x] Manual testing covers all contract scenarios

## Task Generation Rules Applied

1. **From Contracts (POST /api/ocr)**:
   - Contract implementation → T011 (OCR upload handler)
   - Route integration → T012, T013

2. **From Data Model**:
   - UploadConfig → T002
   - ImageFileInfo → T006
   - ValidationResult → T007
   - ValidationError → T003

3. **From Research Decisions**:
   - Dependencies → T001
   - Image validation → T008
   - Error handling → T015, T017

4. **From Quickstart Scenarios**:
   - Manual testing → T021-T025
   - Documentation → T029, T030

## Notes
- **Non-TDD Approach**: Implementation tasks come before testing (T011-T020 before T021-T025)
- **[P] tasks**: Different files, no dependencies, can run concurrently
- **Environment Config**: Handled in T002, T018, T019 for complete configuration management
- **Error Handling**: Comprehensive approach with custom types and proper HTTP responses
- **Manual Testing**: Covers all contract scenarios without automated test dependencies

---

**Total Tasks**: 30
**Estimated Completion**: 6-8 hours for full implementation
**Ready for Execution**: All tasks are specific and immediately actionable