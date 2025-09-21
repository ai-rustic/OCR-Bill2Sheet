# Tasks: Bill Service APIs

**Input**: Design documents from `/specs/003-bill-service-apis/`
**Prerequisites**: plan.md (✓), research.md (✓), data-model.md (✓), contracts/ (✓)

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Tech stack: Rust 1.75+, Axum 0.8.4, SQLx 0.8.6, PostgreSQL
   → Structure: Web application (backend extension)
2. Load optional design documents:
   → data-model.md: ApiResponse wrapper model
   → contracts/: 7 REST endpoints specified
   → research.md: Use existing patterns, skip TDD
3. Generate tasks by category:
   → Setup: API response utilities
   → Core: Individual endpoint handlers
   → Integration: Router configuration
   → Validation: Manual testing via quickstart
4. Apply task rules:
   → Different endpoints = mark [P] for parallel
   → Router integration = sequential (same file)
   → Skip TDD per user request and constitution
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → All 7 endpoints implemented
   → Response wrapper utility created
   → Router properly configured
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Web app**: `backend/src/` structure
- All paths relative to repository root

## Phase 3.1: Setup & Utilities
- [x] T001 Create API response wrapper utilities in `backend/src/api/response.rs`
- [x] T002 [P] Add bill routes module declaration to `backend/src/api/mod.rs`

## Phase 3.2: Core Implementation (Individual Endpoint Handlers)
**Note: Skipping TDD phase per user request and constitutional requirements**
- [x] T003 [P] Implement GET /api/bills endpoint in `backend/src/api/bills.rs` (get_all_bills handler)
- [x] T004 [P] Implement GET /api/bills/{id} endpoint in `backend/src/api/bills.rs` (get_bill_by_id handler)
- [x] T005 [P] Implement POST /api/bills endpoint in `backend/src/api/bills.rs` (create_bill handler)
- [x] T006 [P] Implement PUT /api/bills/{id} endpoint in `backend/src/api/bills.rs` (update_bill handler)
- [x] T007 [P] Implement DELETE /api/bills/{id} endpoint in `backend/src/api/bills.rs` (delete_bill handler)
- [x] T008 [P] Implement GET /api/bills/search endpoint in `backend/src/api/bills.rs` (search_bills handler)
- [x] T009 [P] Implement GET /api/bills/count endpoint in `backend/src/api/bills.rs` (get_bills_count handler)

## Phase 3.3: Integration
- [x] T010 Configure bill routes in main router in `backend/src/main.rs`
- [x] T011 Add bills module export to `backend/src/api/mod.rs`
- [x] T012 Add necessary imports and re-exports for bill handlers

## Phase 3.4: Validation & Polish
- [x] T013 Run cargo clippy and fix any linting issues
- [x] T014 Execute manual testing scenarios from `specs/003-bill-service-apis/quickstart.md`
- [x] T015 [P] Verify Vietnamese text encoding in test requests
- [x] T016 [P] Validate financial precision in decimal calculations

## Dependencies
- T001 (response utilities) blocks T003-T009 (endpoint handlers)
- T002, T011 (module declarations) before T010 (router integration)
- T003-T009 (handlers) before T010 (router integration)
- Implementation (T003-T012) before validation (T013-T016)

## Parallel Example
```
# Launch T003-T009 together after T001 completes:
Task: "Implement GET /bills endpoint in backend/src/api/bills.rs (get_all_bills handler)"
Task: "Implement GET /bills/{id} endpoint in backend/src/api/bills.rs (get_bill_by_id handler)"
Task: "Implement POST /bills endpoint in backend/src/api/bills.rs (create_bill handler)"
Task: "Implement PUT /bills/{id} endpoint in backend/src/api/bills.rs (update_bill handler)"
Task: "Implement DELETE /bills/{id} endpoint in backend/src/api/bills.rs (delete_bill handler)"
Task: "Implement GET /bills/search endpoint in backend/src/api/bills.rs (search_bills handler)"
Task: "Implement GET /bills/count endpoint in backend/src/api/bills.rs (get_bills_count handler)"
```

## Technical Context per Task

### T001: API Response Wrapper Utilities
- Create `ApiResponse<T>` struct with `success`, `data`, `error` fields
- Implement convenience methods: `success(data)`, `error(message)`
- Follow serde serialization patterns from existing health endpoints

### T003-T009: Individual Endpoint Handlers
Each handler should:
- Use Axum's `State<ConnectionPool>` extraction
- Create `BillService` instance from connection pool
- Call appropriate BillService method
- Wrap response in `ApiResponse` format
- Handle errors by converting `ApiError` to `ApiResponse::error`
- Return proper HTTP status codes (200, 201, 404, 400, 500)

### T010: Router Integration
- Add bill routes to existing Axum Router in main.rs
- Routes to add:
  - `/bills` GET and POST
  - `/bills/:id` GET, PUT, DELETE
  - `/bills/search` GET
  - `/bills/count` GET
- Maintain existing health routes and middleware

### T013-T016: Validation Tasks
- T013: Use `cargo clippy` command from CLAUDE.md
- T014: Execute all curl commands from quickstart.md
- T015: Test Vietnamese characters in request bodies
- T016: Verify decimal precision in financial calculations

## Notes
- [P] tasks = different handler functions, no file conflicts
- Skip test creation per constitutional TDD prohibition
- Use existing BillService methods (already implemented)
- Follow Axum patterns from existing health endpoints
- Maintain consistent JSON response structure across all endpoints

## Task Generation Rules Applied
1. **From Contracts**: 7 endpoints → 7 implementation tasks [P]
2. **From Data Model**: ApiResponse wrapper → 1 utility task
3. **From Plan**: Router integration → 3 sequential tasks
4. **From Quickstart**: Manual testing → 4 validation tasks

## Validation Checklist
- [x] All 7 endpoints have implementation tasks
- [x] API response model has utility task
- [x] Implementation before integration
- [x] Parallel tasks truly independent (different handler functions)
- [x] Each task specifies exact file path
- [x] No TDD tasks per user request and constitution