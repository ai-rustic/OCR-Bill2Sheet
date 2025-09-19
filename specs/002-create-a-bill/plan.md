# Implementation Plan: Create Bill Table and Migration

**Branch**: `002-create-a-bill` | **Date**: 2025-09-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-create-a-bill/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → Loaded: Bill table creation with 14 fields for OCR invoice system
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Project Type: web (backend focused, Rust + PostgreSQL)
   → Structure Decision: Option 2 (Web application backend)
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → No violations identified - database schema fits within constitution
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → All requirements clearly specified, no unknowns to resolve
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
7. Re-evaluate Constitution Check section
   → No new violations after design phase
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

## Summary
Create a comprehensive bill table with 14 fields to store Vietnamese invoice data including seller information, item details, pricing, and VAT calculations. The implementation uses SQLx migrations in Rust backend with PostgreSQL, following constitutional requirements for Axum + SQLx architecture with compile-time query validation.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1
**Storage**: PostgreSQL (bill_ocr database)
**Testing**: cargo test && cargo clippy
**Target Platform**: Linux server
**Project Type**: web - backend focused database schema
**Performance Goals**: Standard CRUD operations for bill data
**Constraints**: No API required for this feature, TDD explicitly omitted
**Scale/Scope**: Bill table to support OCR processing workflow

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Backend Simplicity with Axum + SQLx**: Migration uses SQLx with PostgreSQL
✅ **Strict Technology Stack Enforcement**: Backend uses Rust, SQLx, PostgreSQL
✅ **TDD is Explicitly Prohibited**: No test scaffolding required per spec
✅ **No frontend components**: Database schema only, no UI validation needed

**Status**: PASS - No constitutional violations identified

## Project Structure

### Documentation (this feature)
```
specs/002-create-a-bill/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (backend focus)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
├── migrations/          # SQLx migrations for bill table
└── tests/
```

**Structure Decision**: Option 2 - Web application backend (Rust + PostgreSQL focus)

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - No NEEDS CLARIFICATION items in specification
   - All dependencies are clearly defined in CLAUDE.md
   - Database schema is fully specified

2. **Generate and dispatch research agents**:
   - SQLx migration best practices for PostgreSQL
   - PostgreSQL data types for Vietnamese invoice requirements
   - NUMERIC precision handling for financial calculations

3. **Consolidate findings** in `research.md`

**Output**: research.md with migration strategy and PostgreSQL best practices

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Bill entity with 14 fields for Vietnamese invoices
   - Data type mapping from specification to PostgreSQL
   - Constraints and indexes for optimal performance

2. **Generate API contracts** from functional requirements:
   - No API contracts required per specification
   - Focus on database schema definition

3. **Generate contract tests** from contracts:
   - No API tests required (TDD prohibited by constitution)
   - Schema validation via SQLx compile-time checks

4. **Extract test scenarios** from user stories:
   - Database migration success scenarios
   - Data insertion and retrieval validation

5. **Update agent file incrementally**:
   - Update CLAUDE.md with current bill table implementation
   - Add migration patterns and SQLx usage

**Output**: data-model.md, quickstart.md, updated CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate SQLx migration creation tasks
- Database schema validation tasks
- Migration rollback testing tasks

**Ordering Strategy**:
- Migration file creation
- Schema validation
- Migration execution testing
- Rollback procedure validation

**Estimated Output**: 8-10 numbered, ordered tasks in tasks.md focused on database migration

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (SQLx migration creation and testing)
**Phase 5**: Validation (migration execution, schema verification)

## Complexity Tracking
*No constitutional violations identified - this section remains empty*

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