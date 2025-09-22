# Implementation Plan: Modify /api/ocr endpoint to response in SSE (Server-Sent Events)

**Branch**: `005-modify-api-ocr` | **Date**: 2025-09-22 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-modify-api-ocr/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   ✅ Feature spec loaded successfully
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   ✅ Project Type: web (backend only - Rust/Axum API modification)
   ✅ Structure Decision: Using existing backend structure
3. Fill the Constitution Check section based on the content of the constitution document.
   ✅ Constitution loaded and analyzed
4. Evaluate Constitution Check section below
   ✅ No violations - backend-only Axum modification compliant
   ✅ Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   ⏳ Starting research phase
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
   ⏳ Pending research completion
7. Re-evaluate Constitution Check section
   ⏳ Pending design completion
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
   ⏳ Pending previous phases
9. ✅ STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Primary requirement: Modify the existing POST /api/ocr endpoint to respond with Server-Sent Events (SSE) instead of JSON. The endpoint must stream real-time validation progress events for each uploaded image while maintaining the existing multipart/form-data upload functionality and validation logic. No backward compatibility with JSON responses required.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7
**Storage**: PostgreSQL (bill_ocr database) - for configuration only, no new data models needed
**Testing**: cargo test && cargo clippy (per CLAUDE.md)
**Target Platform**: Linux server (existing backend)
**Project Type**: web (backend modification only)
**Performance Goals**: Real-time streaming during image validation process
**Constraints**: Maintain existing file size and count limits, handle client disconnections gracefully
**Scale/Scope**: Existing /api/ocr endpoint modification with 8 new event types

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Backend Simplicity with Axum + SQLx**: Feature modifies existing Axum endpoint
✅ **Strict Technology Stack Enforcement**: Using existing Rust/Axum/SQLx stack
✅ **TDD is Explicitly Prohibited**: User specified "skip TDD process" - compliant
✅ **No Frontend Changes**: SSE is backend response format change only
✅ **SQLx Usage**: Existing endpoint uses SQLx patterns, no new database access needed

**Result**: PASS - No constitutional violations detected

## Project Structure

### Documentation (this feature)
```
specs/005-modify-api-ocr/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (existing structure)
backend/
├── src/
│   ├── models/          # Existing: ImageFileInfo, ValidationResult, etc.
│   ├── services/        # Existing: image_validation
│   └── api/             # MODIFY: ocr.rs endpoint for SSE
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/
```

**Structure Decision**: Option 2 (Web application) - modifying existing backend structure

## Phase 0: Outline & Research

### Research Tasks Identified:
1. **SSE Implementation in Axum**: Best practices for Server-Sent Events with Axum framework
2. **Event Stream Management**: Patterns for streaming events during file processing
3. **Client Disconnection Handling**: Graceful handling of SSE stream interruptions
4. **Event Format Standardization**: SSE event structure and naming conventions
5. **Error Handling in Streams**: How to send error events through SSE without breaking stream

### Research Execution:
**Completed**: research.md created with comprehensive SSE implementation findings

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

### 1. Entity Extraction → data-model.md
✅ **Completed**: ProcessingEvent enum with 8 event types
✅ **Validation rules**: Event sequence and consistency rules defined
✅ **Entity relationships**: Event flow and state management documented

### 2. API Contracts Generation
✅ **Completed**: contracts/sse-api-contract.md
✅ **SSE specification**: Complete event format and HTTP contract
✅ **Integration examples**: cURL and JavaScript client examples

### 3. Implementation Guide
✅ **Completed**: quickstart.md with step-by-step implementation
✅ **Code examples**: Complete handler replacement and model definitions
✅ **Testing procedures**: Verification checklist and troubleshooting

### 4. Agent Context Update
✅ **Completed**: CLAUDE.md updated with SSE implementation context
✅ **Technology stack**: Added SSE-specific dependencies and patterns
✅ **Constitutional compliance**: No violations detected

**Output**: data-model.md, contracts/sse-api-contract.md, quickstart.md, CLAUDE.md updated

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate implementation tasks from design artifacts:
  * Dependency updates (Cargo.toml modifications)
  * Model creation (SSE event types and error codes)
  * Handler replacement (OCR endpoint SSE conversion)
  * State management updates (application state extension)
  * Testing verification (contract compliance testing)

**Specific Task Categories**:
1. **Setup Tasks** [P]: Add SSE dependencies to Cargo.toml
2. **Model Tasks** [P]: Create SSE event models and error types
3. **Handler Tasks**: Replace JSON response with SSE streaming logic
4. **State Tasks**: Extend application state with event broadcasting
5. **Integration Tasks**: Update main.rs router and state initialization
6. **Verification Tasks**: Test SSE events match contract specification

**Ordering Strategy**:
- Dependencies before implementation (Cargo.toml first)
- Models before services (event types before handlers)
- Core logic before integration (handlers before router updates)
- Implementation before verification (code before tests)
- Mark [P] for parallel execution where files don't conflict

**Estimated Task Breakdown**:
- T001-T003: Dependency and model setup (parallel)
- T004-T006: Core SSE handler implementation (sequential)
- T007-T008: Application state and routing integration (sequential)
- T009-T012: Contract verification and testing (parallel possible)

**Total Estimated Output**: 12-15 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitutional violations identified*

No entries required - implementation follows existing Axum/SQLx patterns with SSE streaming addition.

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
