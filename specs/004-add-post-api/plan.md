# Implementation Plan: OCR Image Upload API Endpoint

**Branch**: `004-add-post-api` | **Date**: 2025-09-20 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-add-post-api/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
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
Primary requirement: Create a POST /api/ocr endpoint that accepts multiple image uploads via multipart/form-data with configurable size and count limits from environment variables. The endpoint validates images but does not implement storage or persistence - only request handling and validation logic.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7
**Storage**: PostgreSQL (bill_ocr database) - not used for image storage in this feature
**Testing**: cargo test && cargo clippy
**Target Platform**: Linux server
**Project Type**: web - backend only for this feature
**Performance Goals**: Handle multiple image uploads efficiently with validation
**Constraints**: Environment-configurable limits, multipart/form-data handling, no persistence
**Scale/Scope**: Single API endpoint with validation logic

**User Context**: Make plan to implement this spec, Skip TDD

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Backend Simplicity with Axum + SQLx**: Using Axum framework as required
✅ **Strict Technology Stack Enforcement**: Using Rust, Axum backend stack
✅ **TDD is Explicitly Prohibited**: Skipping TDD as requested, focusing on implementation-first
✅ **Backend Requirements**: Will use connection pooling, environment-based configuration
✅ **No constitutional violations detected**

## Project Structure

### Documentation (this feature)
```
specs/004-add-post-api/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (backend focus for this feature)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/ (exists but not modified in this feature)
```

**Structure Decision**: Option 2 (Web application) - existing backend/ directory structure

## Phase 0: Outline & Research

### Research Required
1. **Axum multipart/form-data handling**: Research best practices for file upload handling in Axum
2. **Environment variable configuration**: Investigate dotenvy usage for configurable limits
3. **Image validation**: Research image format detection and validation techniques in Rust
4. **Error handling patterns**: Review Axum error response patterns for validation failures

### Research Tasks
- Task: "Research Axum multipart form data handling for image uploads"
- Task: "Find best practices for environment-based configuration in Rust backend"
- Task: "Research image format validation and file size checking in Rust"
- Task: "Find error handling patterns for file upload validation in Axum"

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

### Data Models Required
- **UploadRequest**: Structure for incoming multipart request
- **ValidationResult**: Success/failure response structure
- **ConfigSettings**: Environment-based configuration struct

### API Contracts Required
- **POST /api/ocr**: Accept multipart/form-data with multiple images
  - Request: multipart/form-data with image files
  - Response: JSON with validation results
  - Error responses: JSON with error details

### Contract Tests
- Test file upload validation logic
- Test environment configuration handling
- Test error response formats

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Environment configuration setup task
- Multipart handling implementation task
- Image validation logic task
- Error handling and response task
- Integration with existing Axum server task

**Ordering Strategy**:
- Configuration setup first
- Core validation logic
- Axum route integration
- Error handling implementation

**Estimated Output**: 8-12 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitutional violations detected - this section left empty*

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
- [x] Complexity deviations documented (none found)

**Artifacts Generated**:
- [x] research.md - Axum multipart handling research complete
- [x] data-model.md - Entity design and validation rules
- [x] contracts/api-contract.md - POST /api/ocr endpoint specification
- [x] quickstart.md - Implementation guide and testing steps
- [x] CLAUDE.md - Updated with new feature context

---
*Based on Constitution v1.0.0 - See `/.specify/memory/constitution.md`*