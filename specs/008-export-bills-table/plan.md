# Implementation Plan: Export Bills Table

**Branch**: `008-export-bills-table` | **Date**: 2025-09-24 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/008-export-bills-table/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → ✅ Feature spec loaded successfully
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Project Type: web (backend + frontend)
   → Structure Decision: Option 2 (backend/frontend separation)
3. Fill the Constitution Check section based on constitution document
   → Backend: Axum + SQLx compliance required
   → Frontend: Shadcn UI components required (for UI elements if needed)
4. Evaluate Constitution Check section below
   → ✅ No violations - backend export feature aligns with constitution
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → ✅ Research complete: CSV/XLSX libraries selected, streaming patterns defined
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
   → ✅ Design complete: API contracts, data models, test specs, quickstart guide
7. Re-evaluate Constitution Check section
   → ✅ Post-Design Constitution Check: PASS - no violations
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
   → Ready for task generation planning
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Primary requirement: Export all bills from database as CSV or XLSX files via GET endpoint with format parameter. Technical approach: Rust backend using CSV writer crate and Excel generation library, with proper UTF-8/BOM encoding for Vietnamese text.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6, csv crate, xlsx-generation crate (TBD)
**Storage**: PostgreSQL (existing bill_ocr database)
**Testing**: cargo test, cargo clippy
**Target Platform**: Linux server / Windows development
**Project Type**: web - backend API endpoint only
**Performance Goals**: Handle full dataset export efficiently
**Constraints**: UTF-8 with BOM for CSV, UTF-8 for XLSX, Vietnamese text support
**Scale/Scope**: Export all bills (current dataset size unknown, design for scalability)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Backend Axum + SQLx Compliance**: Export endpoint will use Axum handlers with SQLx query macros
✅ **SQLx Compile-time Validation**: All database queries will use `query!` or `query_as!` macros
✅ **Connection Pooling**: Will use existing connection pool from current architecture
✅ **No Custom UI Components**: This is a backend-only API feature
✅ **Technology Stack Enforcement**: Rust backend with approved dependencies only

**Status**: ✅ PASS - No constitutional violations identified

## Project Structure

### Documentation (this feature)
```
specs/008-export-bills-table/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (backend + frontend)
backend/
├── src/
│   ├── models/          # Bill model (existing)
│   ├── services/        # BillService + new ExportService
│   └── api/             # New export endpoint
└── tests/

frontend/
├── src/
│   ├── components/      # Potentially export button component
│   ├── pages/           # Potentially bills management page
│   └── services/        # API client for export endpoint
└── tests/
```

**Structure Decision**: Option 2 (Web application) - backend API with potential frontend integration

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - Best CSV generation crate for Rust (csv vs alternatives)
   - Best XLSX generation crate for Rust (rust_xlsxwriter vs alternatives)
   - UTF-8 with BOM handling for CSV files
   - Memory-efficient streaming for large datasets
   - File response patterns in Axum

2. **Generate and dispatch research agents**:
   ```
   Task: "Research CSV generation libraries for Rust with UTF-8 BOM support"
   Task: "Research XLSX generation libraries for Rust with UTF-8 encoding"
   Task: "Find best practices for file streaming responses in Axum"
   Task: "Research memory-efficient data export patterns for large datasets"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all technical unknowns resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Bill entity (existing with 14 fields)
   - ExportFormat enum (CSV, XLSX)
   - ExportRequest parameters
   - File response structure

2. **Generate API contracts** from functional requirements:
   - GET /api/bills/export?format={csv|xlsx} → file download
   - Error responses for invalid format
   - Output OpenAPI schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - Test CSV format export
   - Test XLSX format export
   - Test invalid format error
   - Test empty dataset handling

4. **Extract test scenarios** from user stories:
   - Export bills as CSV with data
   - Export bills as XLSX with data
   - Export with empty database
   - Handle invalid format parameter

5. **Update CLAUDE.md incrementally**:
   - Add CSV and XLSX export dependencies
   - Add export service patterns
   - Update recent changes

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base structure
- Generate tasks from Phase 1 artifacts (contracts/, data-model.md, quickstart.md)
- **Contract-driven approach**: Each API contract → corresponding test task
- **Service-layer tasks**: Export service with CSV/XLSX generation capability
- **Handler-layer tasks**: Axum endpoint with proper response headers
- **Integration tasks**: Dependency setup (csv, unicode-bom, rust_xlsxwriter)

**Specific Task Categories**:
1. **Setup & Dependencies**: Add export crates to Cargo.toml
2. **Data Models**: ExportFormat enum, ExportParams struct, ExportResponse struct
3. **Service Layer**: ExportService with CSV/XLSX generation methods
4. **Handler Layer**: GET /api/bills/export endpoint with format validation
5. **Contract Tests**: Test CSV export, XLSX export, error cases, Vietnamese text
6. **Integration Tests**: End-to-end export scenarios from quickstart.md

**Ordering Strategy** (Dependency-first):
1. Dependencies and project setup
2. Data models and types (foundation)
3. Service layer implementation (business logic)
4. Handler layer (HTTP interface)
5. Contract tests (API validation)
6. Integration tests (full scenarios)
7. Documentation updates

**Parallelization Opportunities [P]**:
- CSV and XLSX service methods can be implemented in parallel
- Contract test files can be created in parallel
- Data model structs are independent

**Estimated Output**: 18-22 numbered, dependency-ordered tasks in tasks.md

**Constitutional Alignment**:
- All tasks follow SQLx + Axum patterns
- No TDD scaffolding (implementation-first approach)
- Focus on speed and prototype delivery
- UTF-8 Vietnamese text support throughout

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitutional violations identified - table remains empty*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| _None_ | _N/A_ | _N/A_ |

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none required)

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*