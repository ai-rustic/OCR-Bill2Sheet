# Implementation Plan: Backend Database Connection

**Branch**: `HEAD` | **Date**: 2025-09-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/HEAD/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → ✓ COMPLETE: Feature spec loaded and analyzed
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → ✓ COMPLETE: Project Type identified as web application
   → ✓ COMPLETE: Structure Decision set to Option 2 (backend/frontend)
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Connect the Rust backend to PostgreSQL database "bill_ocr" using Axum + SQLx architecture with connection pooling and environment-based configuration. No table creation or model modifications required - work with existing database schema.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7
**Storage**: PostgreSQL (existing bill_ocr database)
**Testing**: cargo test (standard Rust testing)
**Target Platform**: Linux/Windows server environments
**Project Type**: web - determines source structure (backend/frontend separation)
**Performance Goals**: Connection pooling for concurrent request handling
**Constraints**: No table/model creation, use existing schema, SQLx compile-time validation required
**Scale/Scope**: Single database connection with connection pool, environment-driven configuration

**User-provided details**: Lên kế hoạch connect cho tôi backend với database bill_ocr, Ko tạo bảng hay model
(Translation: Plan backend connection to bill_ocr database, don't create tables or models)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**✓ UI-Driven Development (Shadcn-First)**: N/A - Backend-only feature
**✓ Component Validation with mcp**: N/A - Backend-only feature
**✓ TDD is Explicitly Prohibited**: ✓ PASS - No test scaffolding planned, focus on implementation first
**✓ Backend Simplicity with Axum + SQLx**: ✓ PASS - Using required Axum + SQLx stack
**✓ Strict Technology Stack Enforcement**: ✓ PASS - Backend uses Rust, Axum, SQLx, PostgreSQL as required

**Constitution Compliance**: ✓ FULL COMPLIANCE - No violations detected

## Project Structure

### Documentation (this feature)
```
specs/HEAD/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (backend + frontend detected)
backend/
├── src/
│   ├── models/          # Database entity structs
│   ├── services/        # Business logic layer
│   ├── api/            # HTTP route handlers
│   └── config/         # Database configuration
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/
```

**Structure Decision**: Option 2 - Web application (existing backend/ directory confirms this)

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - Database connection best practices with SQLx + PostgreSQL
   - Environment configuration patterns in Rust
   - Connection pooling configuration for bill_ocr database

2. **Generate and dispatch research agents**:
   ```
   Task: "Research SQLx PostgreSQL connection patterns for bill_ocr database"
   Task: "Find best practices for Axum + SQLx connection pooling"
   Task: "Research environment configuration for database URLs in Rust"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with database connection patterns resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Database connection configuration struct
   - Connection pool management
   - Environment variable mapping

2. **Generate API contracts** from functional requirements:
   - Health check endpoint for database connectivity
   - Configuration validation endpoints
   - Output database connection contracts to `/contracts/`

3. **Generate contract tests** from contracts:
   - Database connectivity test
   - Environment configuration test
   - Connection pool behavior test

4. **Extract test scenarios** from user stories:
   - Startup connection scenario
   - Environment configuration scenario
   - Graceful failure scenario

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/powershell/update-agent-context.ps1 -AgentType claude`
   - Add SQLx + PostgreSQL context
   - Update backend connection knowledge

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from database connection requirements
- Connection configuration task [P]
- Environment setup task [P]
- Connection pool implementation task
- Health check endpoint task
- Integration tests for connectivity

**Ordering Strategy**:
- Configuration before connection
- Connection before pooling
- Pooling before health checks
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 8-12 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, database connectivity validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

No constitutional violations detected - table empty.

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
- [x] Complexity deviations documented (none required)

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*