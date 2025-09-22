# Implementation Plan: Gemini OCR Integration

**Branch**: `006-implement-gemini-ocr` | **Date**: 2025-09-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-implement-gemini-ocr/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   ✓ Loaded feature spec successfully
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   ✓ Identified Rust/Axum backend project
   ✓ Found NEEDS CLARIFICATION items
3. Fill the Constitution Check section based on the content of the constitution document.
   ✓ Backend-only feature, Shadcn UI not applicable
4. Evaluate Constitution Check section below
   ✓ No violations - pure backend enhancement
   ✓ Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → Ready to research Gemini API integration patterns
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
7. Re-evaluate Constitution Check section
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Integrate Google Gemini API for OCR processing of Vietnamese bill images through existing `/api/ocr` endpoint. System will process uploaded images sequentially, extract structured bill data matching database schema fields, and return JSON responses. Implementation requires Rust Axum backend integration with Gemini API client, environment configuration management, and structured response handling.

## Technical Context
**Language/Version**: Rust 1.75+ (edition = "2024")
**Primary Dependencies**: Axum 0.8.4, SQLx 0.8.6, Tokio 1.47.1, dotenvy 0.15.7, reqwest (HTTP client for Gemini API)
**Storage**: PostgreSQL (bills table with 14 fields for Vietnamese invoices)
**Testing**: cargo test && cargo clippy
**Target Platform**: Backend API server
**Project Type**: web (backend only enhancement)
**Performance Goals**: Temporarily ignored for MVP implementation
**Constraints**: Temporarily ignored for MVP implementation
**Scale/Scope**: Enhancement to existing OCR endpoint for Vietnamese bill processing

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Axum + SQLx Backend Requirements**:
- ✅ Using Axum as HTTP framework
- ✅ Using SQLx for async database access with compile-time validation
- ✅ PostgreSQL as primary data store with connection pooling
- ✅ Environment-based configuration via .env variables

**Technology Stack Enforcement**:
- ✅ Backend: Rust + Axum + SQLx + PostgreSQL maintained
- ✅ JSON over HTTP interfacing pattern
- N/A Frontend components (backend-only feature)

**No TDD Prohibition**:
- ✅ Implementation-first approach for speed and prototype delivery
- ✅ No test scaffolding required during development

**Result**: PASS - No constitutional violations identified

## Project Structure

### Documentation (this feature)
```
specs/006-implement-gemini-ocr/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (backend enhancement)
backend/
├── src/
│   ├── models/          # Existing Bill model
│   ├── services/        # New: gemini_service.rs
│   ├── api/            # Enhanced: ocr_handler.rs
│   └── config/         # Enhanced: .env with Gemini config
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/
```

**Structure Decision**: Option 2 (Web application) - Enhancement to existing backend/src structure

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - NEEDS CLARIFICATION: Gemini API client library → research available Rust clients
   - NEEDS CLARIFICATION: Performance targets → research API rate limits and processing times
   - NEEDS CLARIFICATION: API constraints → research image limits, timeout behavior, error handling

2. **Generate and dispatch research agents**:
   ```
   Task: "Research Gemini API Rust client libraries for image OCR processing"
   Task: "Find best practices for Gemini API integration in Rust/Axum applications"
   Task: "Research Gemini API rate limits, image constraints, and error handling patterns"
   Task: "Investigate structured output capabilities for Vietnamese text extraction"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - GeminiOCRRequest: image data, processing options
   - GeminiOCRResponse: extracted bill fields, confidence scores
   - OCRError: error types and handling strategies
   - Mapping to existing Bill entity fields

2. **Generate API contracts** from functional requirements:
   - Enhanced POST /api/ocr endpoint with Gemini integration
   - Request: multipart/form-data with images
   - Response: structured JSON with Bill-compatible fields
   - Error responses for API failures, invalid images, timeouts

3. **Generate contract tests** from contracts:
   - Test file for enhanced OCR endpoint
   - Assert request/response schemas match Bill table structure
   - Tests must fail (no Gemini implementation yet)

4. **Extract test scenarios** from user stories:
   - Clear Vietnamese bill image → complete structured data
   - Multiple images → sequential processing with ordered results
   - Poor quality image → partial data with confidence indicators
   - API failure → graceful error response

5. **Update agent file incrementally**:
   - Run update-agent-context.ps1 for Claude Code
   - Add Gemini API integration to recent changes
   - Include Vietnamese text processing capabilities
   - Preserve manual additions between markers

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load tasks-template.md as base
- Generate tasks from Gemini API integration requirements
- Each contract → contract test task [P]
- Gemini service → service creation task [P]
- OCR handler enhancement → API integration task
- Configuration → environment setup task

**Ordering Strategy**:
- Implementation-first (per constitution): Service before tests
- Dependency order: Config → Service → Handler → Integration
- Mark [P] for parallel execution where applicable

**Estimated Output**: 15-20 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

No constitutional violations identified. Feature aligns with:
- Axum + SQLx backend architecture
- Implementation-first approach (no TDD)
- Environment-based configuration
- JSON over HTTP interfacing

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
- [x] All NEEDS CLARIFICATION resolved (documented in research.md)
- [x] Complexity deviations documented (none required)

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*