# Tasks: Backend Database Connection

**Input**: Design documents from `/specs/HEAD/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → ✓ COMPLETE: Tech stack (Rust+Axum+SQLx), web structure (backend/)
2. Load optional design documents:
   → ✓ data-model.md: DatabaseConfig, ConnectionPool, HealthStatus entities
   → ✓ contracts/: health.yaml, database.yaml for API endpoints
   → ✓ research.md: SQLx PgPool, Axum State pattern decisions
3. Generate tasks by category:
   → Setup: Rust project init, dependencies, environment
   → Core: config models, connection pool, health service
   → Integration: DB connection, middleware, error handling
   → Polish: environment validation, performance, docs
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Implementation-first approach (NO TDD as per constitution)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → ✓ All entities have models
   → ✓ All endpoints implemented
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Web app**: `backend/src/` (per plan.md structure)
- All paths are absolute from repository root

## Phase 3.1: Setup

- [x] T001 Create backend project structure per implementation plan
- [x] T002 Initialize Rust project with Axum, SQLx, Tokio dependencies in backend/Cargo.toml
- [x] T003 [P] Configure environment setup with .env file in backend/.env

## Phase 3.2: Core Implementation

- [x] T004 [P] DatabaseConfig struct in backend/src/config/database.rs
- [x] T005 [P] ConnectionPool wrapper in backend/src/config/mod.rs
- [x] T006 [P] DatabaseError enum in backend/src/config/database.rs
- [x] T007 [P] HealthStatus model in backend/src/services/health.rs
- [x] T008 Environment configuration loader in backend/src/config/database.rs
- [x] T009 Connection pool initialization in backend/src/config/mod.rs
- [x] T010 Health check service implementation in backend/src/services/health.rs

## Phase 3.3: API Endpoints

- [x] T011 GET /health endpoint in backend/src/api/health.rs
- [x] T012 GET /health/detail endpoint in backend/src/api/health.rs
- [x] T013 API module organization in backend/src/api/mod.rs
- [x] T014 Axum router setup with endpoints in backend/src/main.rs

## Phase 3.4: Integration

- [x] T015 Database connection initialization in main.rs
- [x] T016 Axum State integration for connection pool in backend/src/main.rs
- [x] T017 Error handling middleware in backend/src/api/mod.rs
- [x] T018 Request logging setup in backend/src/main.rs

## Phase 3.5: Polish

- [x] T019 [P] Update quickstart.md validation
- [x] T020 Remove code duplication across modules
- [x] T021 Run manual testing scenarios from quickstart.md

## Dependencies

**Setup Phase**:
- T001 → T002 → T003

**Core Implementation**:
- T004, T005, T006, T007 can run in parallel (different files)
- T008 depends on T004 (same file backend/src/config/database.rs)
- T009 depends on T005 (same file backend/src/config/mod.rs)
- T010 depends on T007 (same file backend/src/services/health.rs)

**API Endpoints**:
- T011, T012 sequential (same file backend/src/api/health.rs)
- T013 depends on T011-T012
- T014 depends on T013

**Integration**:
- T015 → T016 → T017 → T018 (sequential, main.rs and related files)

**Polish** (all parallel):
- T019, T020, T021 can run in parallel

## Parallel Execution Examples

### Phase 3.2 - Core Models:
```bash
# Launch T004-T007 together:
Task: "DatabaseConfig struct in backend/src/config/database.rs"
Task: "ConnectionPool wrapper in backend/src/config/mod.rs"
Task: "DatabaseError enum in backend/src/config/database.rs"
Task: "HealthStatus model in backend/src/services/health.rs"
```

### Phase 3.5 - Polish Tasks:
```bash
# Launch T019-T021 together:
Task: "Update quickstart.md validation"
Task: "Remove code duplication across modules"
Task: "Run manual testing scenarios from quickstart.md"
```

## Notes

- **[P] tasks** = different files, no dependencies
- **Constitution compliance**: Implementation-first approach (NO TDD)
- **No test creation** as requested by user
- **No table/model creation** as specified in user requirements
- **Environment-driven configuration** for bill_ocr database connection
- **SQLx compile-time validation** required for all queries
- **Axum State pattern** for connection pool sharing

## Task Generation Rules Applied

1. **From Contracts**:
   - health.yaml → T011, T012 (implementation only)

2. **From Data Model**:
   - DatabaseConfig → T004 (model creation)
   - ConnectionPool → T005 (model creation)
   - HealthStatus → T007 (model creation)
   - DatabaseError → T006 (error handling)

3. **From User Stories**:
   - Database connection initialization → T015 (integration)
   - Environment configuration → T008 (implementation)
   - Graceful failure → Error handling in T006

4. **Ordering**:
   - Setup → Core → Endpoints → Integration → Polish
   - Dependencies prevent parallel execution where files overlap

## Validation Checklist

- [x] All entities have model tasks (T004-T007 → DatabaseConfig, ConnectionPool, HealthStatus, DatabaseError)
- [x] All endpoints implemented (T011-T012 → health, health/detail)
- [x] Implementation-first approach (NO TDD tests created)
- [x] Parallel tasks truly independent (verified file paths)
- [x] Each task specifies exact file path (backend/src/*)
- [x] No task modifies same file as another [P] task (validated)
- [x] Vietnamese user requirement honored: "bỏ qua bước tạo test (TDD)" (skip TDD test creation)