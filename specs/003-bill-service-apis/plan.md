# Implementation Plan: Bill Service APIs

**Branch**: `003-bill-service-apis` | **Date**: 2025-09-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-bill-service-apis/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → Loaded successfully: 13 functional requirements for REST API endpoints
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detected Project Type: web (backend API endpoints)
   → Set Structure Decision: Option 2 (Web application)
3. Fill the Constitution Check section based on constitution document
4. Evaluate Constitution Check section below
   → No violations: Axum + SQLx backend aligns with constitution
   → Update Progress Tracking: Initial Constitution Check ✓
5. Execute Phase 0 → research.md
   → No NEEDS CLARIFICATION to resolve (skipping TDD per user request)
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
7. Re-evaluate Constitution Check section
   → No new violations: Design maintains constitutional compliance
   → Update Progress Tracking: Post-Design Constitution Check ✓
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

## Summary
Primary requirement: Implement 7 RESTful API endpoints for Vietnamese bill/invoice management (CRUD + search + count) with consistent JSON responses and proper HTTP status codes. Technical approach: Extend existing Rust/Axum backend with new bill API routes using established BillService and SQLx patterns.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7
**Storage**: PostgreSQL (bill_ocr database, bills table already exists)
**Testing**: cargo test && cargo clippy (per CLAUDE.md, skipping TDD per user request)
**Target Platform**: Linux server (HTTP REST API)
**Project Type**: web - backend API extension to existing Axum server
**Performance Goals**: Standard REST API performance (< 200ms p95 for CRUD operations)
**Constraints**: Vietnamese text encoding support, financial precision (NUMERIC 18,2), consistent JSON structure
**Scale/Scope**: Small-medium scale (typical invoice management, 7 endpoints)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Backend Simplicity with Axum + SQLx**: Feature extends existing Axum server with new routes
✅ **SQLx query macros**: Will use existing pattern with query_as! and compile-time validation
✅ **Connection pooling**: Will reuse existing ConnectionPool infrastructure
✅ **TDD Explicitly Prohibited**: User explicitly requested "skip TDD process" - aligns with constitution
✅ **No external dependencies**: All APIs use existing tech stack (Axum, SQLx, serde)
✅ **Modular route separation**: Will add new bill routes to existing api module structure

**Gate Status**: PASS - No constitutional violations

## Project Structure

### Documentation (this feature)
```
specs/003-bill-service-apis/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (backend API extension)
backend/
├── src/
│   ├── api/
│   │   ├── mod.rs        # Add bill routes to router
│   │   ├── health.rs     # Existing
│   │   └── bills.rs      # NEW: Bill API handlers
│   ├── models/
│   │   ├── mod.rs        # Existing
│   │   └── bill.rs       # Existing Bill + CreateBill structs
│   ├── services/
│   │   ├── mod.rs        # Existing
│   │   └── bill_service.rs # Existing BillService with CRUD methods
│   └── main.rs           # Update router to include bill routes
└── tests/
    └── contract/         # API contract tests
```

**Structure Decision**: Option 2 (Web application) - Extending existing backend with new API routes

## Phase 0: Outline & Research

Since user requested "skip TDD process" and all technical dependencies are already established in the existing codebase (Axum, SQLx, PostgreSQL), no research is needed. The implementation will follow existing patterns.

**Research completed**: All technologies and patterns established in current codebase

## Phase 1: Design & Contracts

### Data Model (data-model.md)
- **Entity**: Bill (already exists in backend/src/models/bill.rs)
- **Fields**: 14 fields for Vietnamese invoices (form_no, serial_no, invoice_no, issued_date, seller info, item details, financial calculations)
- **Validation**: Existing SQLx schema validation, financial precision NUMERIC(18,2)

### API Contracts (contracts/)
Will generate OpenAPI specs for 7 endpoints:
- GET /bills - List all bills
- GET /bills/{id} - Get bill by ID
- POST /bills - Create new bill
- PUT /bills/{id} - Update bill
- DELETE /bills/{id} - Delete bill
- GET /bills/search?invoice={pattern} - Search by invoice number
- GET /bills/count - Get total count

### Response Structure
Consistent JSON format:
```json
{
  "success": boolean,
  "data": object | array | null,
  "error": string | null
}
```

### Agent Context Update (CLAUDE.md)
Will add Bill API implementation details to existing CLAUDE.md context.

**Output**: data-model.md, /contracts/*, quickstart.md, updated CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Generate implementation tasks (no tests per constitution and user request)
- Each endpoint → handler implementation task
- API response wrapper → utility task
- Router integration → routing task
- Manual testing → quickstart verification task

**Ordering Strategy**:
- Response utilities first (shared across endpoints)
- Individual endpoint handlers (can be parallel)
- Router integration last
- Manual testing/verification

**Estimated Output**: 10-12 implementation tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (manual testing via quickstart.md, cargo clippy)

## Complexity Tracking
*No constitutional violations requiring justification*

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*