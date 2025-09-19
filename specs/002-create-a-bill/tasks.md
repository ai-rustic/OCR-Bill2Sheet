# Tasks: Create Bill Table and Migration

**Input**: Design documents from `/specs/002-create-a-bill/`
**Prerequisites**: plan.md (✅), research.md (✅), data-model.md (✅), contracts/ (✅)

## Execution Flow (main)
```
1. Load plan.md from feature directory
   ✅ Loaded: Rust + SQLx + PostgreSQL backend implementation
   ✅ Extract: tech stack (SQLx 0.8.6, PostgreSQL), backend structure
2. Load optional design documents:
   ✅ data-model.md: Bills entity with 14 fields → migration tasks
   ✅ contracts/: Database schema contract → validation tasks
   ✅ research.md: SQLx migration strategy → setup tasks
3. Generate tasks by category:
   ✅ Setup: SQLx migration files, environment verification
   ✅ Core: Migration creation, schema validation
   ✅ Integration: Database connection, compile-time validation
   ✅ Polish: Migration testing, rollback validation
4. Apply task rules:
   ✅ Different files = mark [P] for parallel
   ✅ Sequential execution for migration dependencies
   ✅ Skip TDD per constitutional requirements and context
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   ✅ Bill entity has migration
   ✅ Schema validation included
   ✅ Rollback safety verified
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Web app backend**: `backend/migrations/`, `backend/src/`
- Paths assume web application structure per plan.md

## Phase 3.1: Setup and Environment
- [x] T001 Verify PostgreSQL database connection and SQLx CLI installation
- [x] T002 Navigate to backend directory and verify SQLx project configuration
- [x] T003 [P] Create backend/migrations/ directory if not exists

## Phase 3.2: Migration Creation
- [x] T004 Create SQLx migration file: `sqlx migrate add create_bill_table` in backend/
- [x] T005 Write forward migration SQL in backend/migrations/YYYYMMDDHHMMSS_create_bill_table.up.sql
- [x] T006 Write rollback migration SQL in backend/migrations/YYYYMMDDHHMMSS_create_bill_table.down.sql

## Phase 3.3: Schema Implementation
- [x] T007 Implement bills table schema with 14 fields as specified in data-model.md
- [x] T008 Add SERIAL primary key for id field
- [x] T009 [P] Add TEXT fields: form_no, serial_no, invoice_no, seller_name, seller_tax_code, item_name, unit
- [x] T010 [P] Add DATE field: issued_date
- [x] T011 [P] Add NUMERIC(18,2) fields: quantity, unit_price, total_amount, vat_amount
- [x] T012 [P] Add NUMERIC(5,2) field: vat_rate

## Phase 3.4: Migration Execution and Validation
- [x] T013 Run migration: `sqlx migrate run` in backend/
- [x] T014 Verify table creation: `psql $DATABASE_URL -c "\d bills"`
- [x] T015 Test data insertion with Vietnamese text and numeric precision
- [x] T016 Verify all 14 fields accept correct data types

## Phase 3.5: Rust Integration
- [x] T017 Create Bill struct in backend/src/models/bill.rs with sqlx::FromRow derive
- [x] T018 Map PostgreSQL types to Rust types (SERIAL→i32, TEXT→Option<String>, etc.)
- [x] T019 Add compile-time query validation with sqlx::query! macro examples
- [x] T020 Test Rust struct mapping with sample queries

## Phase 3.6: Migration Safety and Rollback
- [x] T021 Test migration rollback: `sqlx migrate revert` in backend/
- [x] T022 Verify table removal after rollback
- [x] T023 Re-apply migration to restore table
- [x] T024 Document migration safety procedures

## Phase 3.7: Validation and Polish
- [x] T025 [P] Validate financial precision with NUMERIC types
- [x] T026 [P] Test Vietnamese text support with UTF-8 characters
- [x] T027 [P] Verify date handling with issued_date field
- [x] T028 Run cargo check to verify SQLx compile-time validation
- [x] T029 Update CLAUDE.md with migration patterns and usage examples
- [x] T030 Document quickstart validation results

## Dependencies
- T001-T003 (Setup) before T004-T006 (Migration creation)
- T004-T006 (Migration files) before T007-T012 (Schema implementation)
- T007-T012 (Schema) before T013-T016 (Execution)
- T013-T016 (Database validation) before T017-T020 (Rust integration)
- T017-T020 (Rust integration) before T021-T024 (Rollback testing)
- Everything before T025-T030 (Final validation)

## Parallel Example
```bash
# Launch T009-T012 together (different field groups):
# Terminal 1: Add TEXT fields to migration
# Terminal 2: Add DATE field to migration
# Terminal 3: Add NUMERIC(18,2) fields to migration
# Terminal 4: Add NUMERIC(5,2) field to migration

# Launch T025-T027 together (different validation types):
# Terminal 1: Test financial precision
# Terminal 2: Test Vietnamese text
# Terminal 3: Test date handling
```

## Context Integration
**Skip TDD Process**: Per constitutional requirements and context directive
- No test scaffolding phase required
- Focus on implementation-first approach
- Validation through SQLx compile-time checks and manual testing
- Database schema validation replaces traditional TDD cycle

## Notes
- [P] tasks = different files or independent field groups
- Migration files are sequential by nature (up → down → verification)
- Commit after each migration milestone
- Constitutional compliance: No TDD, SQLx-only, PostgreSQL focus

## Task Generation Rules Applied

1. **From Data Model**:
   ✅ Bills entity → migration creation tasks (T004-T006)
   ✅ 14 fields → schema implementation tasks (T007-T012)

2. **From Research**:
   ✅ SQLx migration strategy → setup and execution tasks (T001-T003, T013-T016)
   ✅ PostgreSQL types → Rust integration tasks (T017-T020)

3. **From Contracts**:
   ✅ Schema validation → testing and rollback tasks (T021-T024)
   ✅ Database safety → validation tasks (T025-T030)

4. **From Quickstart**:
   ✅ Validation scenarios → manual testing tasks (T015, T025-T027)

## Validation Checklist
*GATE: Checked before execution*

- [x] Bills entity has migration tasks (T004-T012)
- [x] Schema validation included (T013-T016, T025-T027)
- [x] Rollback safety verified (T021-T024)
- [x] Parallel tasks truly independent (different field groups)
- [x] Each task specifies exact file path or command
- [x] No TDD conflicts with constitutional requirements
- [x] SQLx compile-time validation integrated (T019, T028)