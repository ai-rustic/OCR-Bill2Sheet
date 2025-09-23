# Implementation Plan: Integrate Gemini Service to API/OCR

**Branch**: `006-integrate-gemini-service` | **Date**: 2025-09-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-integrate-gemini-service/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → ✅ Feature spec loaded successfully
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → ✅ Project Type: web (frontend+backend)
   → ✅ Structure Decision: Option 2 (Web application)
3. Fill the Constitution Check section based on the content of the constitution document.
   → ✅ Constitution requirements documented
4. Evaluate Constitution Check section below
   → ✅ No violations detected
   → ✅ Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → ✅ Research phase completed
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
   → ✅ Design artifacts generated
7. Re-evaluate Constitution Check section
   → ✅ No new violations
   → ✅ Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
   → ✅ Task planning approach documented
9. STOP - Ready for /tasks command
```

## Summary
Integrate Gemini AI service with the existing OCR API to extract structured bill data from uploaded images. Users upload multiple images via the API, which are processed sequentially through Gemini's structured output API, with real-time results streamed back via Server-Sent Events (SSE). The extracted data will conform to the existing bills database schema.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7, reqwest for HTTP client
**Storage**: PostgreSQL (bill_ocr database) via SQLx
**Testing**: cargo test && cargo clippy
**Target Platform**: Linux server
**Project Type**: web - determines source structure
**Performance Goals**: Sequential image processing, real-time SSE streaming
**Constraints**: Use existing MAX_FILE_SIZE_BYTES, support JPG/PNG/JFIF formats only
**Scale/Scope**: Handle multiple image uploads per request, Gemini API rate limiting

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Backend Requirements**:
- ✅ Use SQLx with compile-time query validation (`query!`, `query_as!`)
- ✅ Connection pooling required for PostgreSQL (bill_ocr database)
- ✅ Environment-based configuration via DATABASE_URL
- ✅ No ORM except SQLx - follow constitution strictly

**Development Workflow**:
- ✅ TDD is PROHIBITED - Implementation-first approach
- ✅ Speed and prototype delivery prioritized
- ✅ No test scaffolding during development

**Technology Stack**:
- ✅ Backend: Rust, Axum, SQLx, PostgreSQL
- ✅ Interfacing: JSON over HTTP
- ✅ Must use connection pooling, environment-configured database URLs

**No violations detected** - All requirements align with constitutional principles.

## Project Structure

### Documentation (this feature)
```
specs/006-integrate-gemini-service/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/
```

**Structure Decision**: Option 2 - Web application (backend + frontend structure detected)

## Phase 0: Outline & Research

Research completed for:
- Gemini AI API integration patterns for structured output
- Server-Sent Events (SSE) implementation with Axum
- Image processing and base64 encoding for API requests
- Rate limiting and error handling strategies
- Vietnamese text OCR considerations

**Output**: research.md with all technical unknowns resolved

## Phase 1: Design & Contracts

Design artifacts created:
1. **data-model.md**: Extracted bill data structure matching database schema
2. **contracts/**: API endpoint specifications for OCR processing with SSE
3. **quickstart.md**: End-to-end testing scenarios
4. **CLAUDE.md**: Updated with new Gemini integration context

**Output**: Complete design documentation and API contracts

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P]
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- Implementation-first order (no TDD per constitution)
- Dependency order: Models before services before API routes
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 20-25 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitutional violations detected*

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) ✅
- [x] Phase 1: Design complete (/plan command) ✅
- [x] Phase 2: Task planning complete (/plan command - describe approach only) ✅
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS ✅
- [x] Post-Design Constitution Check: PASS ✅
- [x] All NEEDS CLARIFICATION resolved ✅
- [x] Complexity deviations documented (none) ✅

**Generated Artifacts**:
- [x] research.md - Gemini API integration patterns
- [x] data-model.md - Entity definitions and data flow
- [x] contracts/ocr-api.yaml - OpenAPI specification
- [x] quickstart.md - End-to-end testing scenarios
- [x] CLAUDE.md - Updated with Gemini context

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*