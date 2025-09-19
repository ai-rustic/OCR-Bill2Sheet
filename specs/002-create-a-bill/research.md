# Research: Bill Table Implementation

**Date**: 2025-09-19
**Scope**: SQLx migration strategy and PostgreSQL best practices for Vietnamese invoice data

## Research Areas

### 1. SQLx Migration Best Practices

**Decision**: Use SQLx CLI with versioned migrations
**Rationale**:
- Compile-time SQL validation with `query!` macros
- Built-in migration rollback support
- Version control friendly with `.sql` files
- Integrates seamlessly with existing Axum + SQLx setup

**Alternatives considered**:
- Manual SQL scripts: Less safe, no compile-time validation
- Diesel ORM: Violates constitution (SQLx required)

### 2. PostgreSQL Data Types for Vietnamese Invoices

**Decision**: Use standard PostgreSQL types with appropriate precision
**Rationale**:
- `SERIAL` for auto-incrementing primary keys
- `TEXT` for Vietnamese text (supports UTF-8 natively)
- `NUMERIC(18,2)` for financial precision (no floating-point errors)
- `NUMERIC(5,2)` for VAT percentages (up to 999.99%)
- `DATE` for issue dates (no time component needed)

**Alternatives considered**:
- `VARCHAR` with limits: Less flexible for Vietnamese text
- `DECIMAL`: PostgreSQL treats as alias to NUMERIC
- `MONEY`: Currency-specific, less flexible for calculations

### 3. Financial Calculation Precision

**Decision**: NUMERIC(18,2) for monetary values, NUMERIC(5,2) for percentages
**Rationale**:
- 18 digits total with 2 decimal places handles up to 999,999,999,999,999.99
- Avoids floating-point precision issues in financial calculations
- Standard practice for invoice systems
- Compatible with Vietnamese Dong currency calculations

**Alternatives considered**:
- `FLOAT`/`DOUBLE`: Precision errors unacceptable for financial data
- Higher precision: Unnecessary overhead for typical invoice amounts

### 4. Migration File Structure

**Decision**: Single migration file with `up.sql` and `down.sql`
**Rationale**:
- SQLx supports both forward and rollback migrations
- Single table creation is atomic operation
- Simple rollback with `DROP TABLE IF EXISTS`
- Follows SQLx conventions

**Alternatives considered**:
- Multiple migration steps: Unnecessary for single table
- Irreversible migration: Violates FR-009 requirement

## Implementation Strategy

### Migration File Location
```
backend/migrations/YYYYMMDDHHMMSS_create_bill_table.sql
```

### Schema Definition
- Primary key: `id SERIAL PRIMARY KEY`
- Text fields: All `TEXT` type for maximum flexibility
- Date field: `DATE` type for issued_date
- Numeric fields: `NUMERIC(18,2)` for monetary values
- VAT rate: `NUMERIC(5,2)` for percentage values

### Indexing Strategy
- Primary key index (automatic)
- Consider composite index on (seller_tax_code, invoice_no) for lookups
- Date range index on issued_date for reporting queries

## Validation Approach

### Compile-time Validation
- Use SQLx `query!` macros to validate schema at compile time
- Type safety ensured through Rust's type system
- SQL syntax validation during cargo build

### Runtime Validation
- PostgreSQL constraints handle data integrity
- Application-level validation for business rules
- Migration rollback testing for deployment safety

## Dependencies Confirmed

From existing CLAUDE.md:
- ✅ SQLx 0.8.6 with postgres feature
- ✅ Database connection via DATABASE_URL environment variable
- ✅ Connection pooling already configured
- ✅ Tokio async runtime compatible

## Conclusion

All technical unknowns resolved. The implementation follows constitutional requirements using SQLx + PostgreSQL with proper financial precision and Vietnamese text support. No additional dependencies required beyond current stack.