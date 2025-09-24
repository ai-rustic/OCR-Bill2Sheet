# Tasks: Export Bills Table

**Input**: Design documents from `/specs/008-export-bills-table/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → ✅ Implementation plan loaded successfully
   → Tech stack: Rust 1.75+, Axum, SQLx, csv, rust_xlsxwriter
2. Load optional design documents:
   → data-model.md: ExportFormat, ExportParams, ExportResponse entities
   → contracts/: GET /api/bills/export endpoint specification
   → research.md: Library decisions and streaming approach
3. Generate tasks by category:
   → Setup: Dependencies, project structure
   → Core: Models, services, endpoints (SKIP TDD per user request)
   → Integration: Database integration, error handling
   → Polish: Testing, documentation
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → SKIP TDD: Implementation-first approach
5. Number tasks sequentially (T001, T002...)
6. Dependencies: Setup → Models → Services → Endpoints → Integration → Polish
7. ✅ All contracts, entities, and endpoints covered
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
Based on plan.md structure: **Web app** - `backend/src/`, `backend/tests/`

## Phase 3.1: Setup & Dependencies ✅ COMPLETED
- [x] T001 Add export dependencies to backend/Cargo.toml (csv = "1.3", unicode-bom = "2.0", rust_xlsxwriter = "0.78", tokio-stream = "0.1", futures-util = "0.3")
- [x] T002 [P] Configure linting to allow export module: Update backend/.cargo/config.toml or rust-analyzer settings
- [x] T003 Create export module structure: backend/src/models/export.rs, backend/src/services/export_service.rs, backend/src/api/export.rs

## Phase 3.2: Data Models (Implementation-First) ✅ COMPLETED
- [x] T004 [P] ExportFormat enum in backend/src/models/export.rs (CSV, XLSX variants with serde support)
- [x] T005 [P] ExportParams struct in backend/src/models/export.rs (format field with validation)
- [x] T006 [P] ExportResponse helper in backend/src/models/export.rs (filename, content_type, content methods)
- [x] T007 [P] ExportError enum in backend/src/models/export.rs (DatabaseError, SerializationError, IoError variants)

## Phase 3.3: Service Layer ✅ COMPLETED
- [x] T008 ExportService CSV generation method in backend/src/services/export_service.rs (bills_to_csv with UTF-8 BOM)
- [x] T009 ExportService XLSX generation method in backend/src/services/export_service.rs (bills_to_xlsx with formatting)
- [x] T010 ExportService database query method in backend/src/services/export_service.rs (get_all_bills with SQLx query_as!)
- [x] T011 ExportService main export method in backend/src/services/export_service.rs (handles format routing and error handling)

## Phase 3.4: API Endpoints
- [x] T012 GET /api/bills/export handler in backend/src/api/export.rs (validate params, call service, return file response)
- [x] T013 HTTP response headers setup in backend/src/api/export.rs (Content-Type, Content-Disposition, Cache-Control)
- [x] T014 Error handling middleware integration in backend/src/api/export.rs (400 for invalid format, 500 for errors)
- [x] T015 Register export routes in backend/src/main.rs or backend/src/api/mod.rs

## Phase 3.5: Database Integration
- [x] T016 Update Bill model imports in backend/src/models/bill.rs (ensure serde::Serialize is available)
- [x] T017 Test database connection for export queries using existing connection pool
- [x] T018 [P] Add field transformation utilities in backend/src/services/export_service.rs (handle Option<> fields, Vietnamese text)

## Phase 3.6: File Generation & Headers
- [x] T019 [P] Implement CSV column headers in backend/src/services/export_service.rs (Vietnamese + English bilingual headers)
- [x] T020 [P] Implement XLSX worksheet setup in backend/src/services/export_service.rs (header formatting, auto-fit)
- [x] T021 Add timestamp filename generation in backend/src/services/export_service.rs (bills_export_YYYYMMDD_HHMMSS format)

## Phase 3.7: Integration & Polish
- [ ] T022 End-to-end testing using scenarios from quickstart.md (CSV export, XLSX export, error handling)
- [ ] T023 Performance validation with large dataset (verify streaming memory usage)
- [ ] T024 [P] Vietnamese text encoding validation (test with sample Vietnamese bills data)
- [ ] T025 [P] Update backend/src/lib.rs to export new modules (pub mod models, pub mod services, pub mod api)

## Dependencies
- T001 (dependencies) before all other tasks
- T003 (module structure) before T004-T007, T008-T011, T012-T015
- T004-T007 (models) before T008-T011 (services)
- T008-T011 (services) before T012-T015 (endpoints)
- T016-T018 (DB integration) before T022-T024 (testing)
- Implementation tasks before polish (T022-T025)

## Parallel Execution Examples

### Setup Phase (can run together after T001):
```bash
# T002 and T003 can run in parallel:
Task: "Configure linting for export module in backend/.cargo/config.toml"
Task: "Create export module files: backend/src/models/export.rs, backend/src/services/export_service.rs, backend/src/api/export.rs"
```

### Models Phase (can run together after T003):
```bash
# T004-T007 can run in parallel:
Task: "Create ExportFormat enum in backend/src/models/export.rs"
Task: "Create ExportParams struct in backend/src/models/export.rs"
Task: "Create ExportResponse helper in backend/src/models/export.rs"
Task: "Create ExportError enum in backend/src/models/export.rs"
```

### Polish Phase (can run together):
```bash
# T024-T025 can run in parallel:
Task: "Validate Vietnamese text encoding with sample data"
Task: "Update backend/src/lib.rs module exports"
```

## File-Specific Task Groups

### backend/src/models/export.rs
- T004, T005, T006, T007 (can run in parallel within this file)

### backend/src/services/export_service.rs
- T008, T009, T010, T011 (sequential - same file)
- T018, T019, T021 (sequential - same file)

### backend/src/api/export.rs
- T012, T013, T014 (sequential - same file)

## Notes
- **No TDD**: Implementation-first approach as requested - tests come after implementation
- **[P] tasks**: Different files, no dependencies between them
- **Streaming approach**: All file generation uses memory-efficient streaming patterns
- **Vietnamese text**: UTF-8 BOM for CSV, UTF-8 for XLSX ensures proper rendering
- **Constitutional compliance**: Uses SQLx query macros, Axum handlers, no TDD requirement

## Task Generation Rules Applied
1. **From Contracts**: GET /api/bills/export → T012-T015 (endpoint implementation)
2. **From Data Model**: 4 entities (ExportFormat, ExportParams, ExportResponse, ExportError) → T004-T007
3. **From Research**: Library integration → T001 (dependencies), T008-T009 (CSV/XLSX generation)
4. **From Implementation Plan**: Service layer → T008-T011, streaming approach → T018-T021

## Validation Checklist ✅
- [x] All contracts (GET /api/bills/export) have corresponding implementation tasks
- [x] All entities (ExportFormat, ExportParams, ExportResponse, ExportError) have model tasks
- [x] Implementation-first approach (no test scaffolding before implementation)
- [x] Parallel tasks are truly independent ([P] tasks use different files)
- [x] Each task specifies exact file path
- [x] No [P] task modifies same file as another [P] task
- [x] Dependencies properly ordered: Setup → Models → Services → Endpoints → Polish