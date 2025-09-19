# Contracts: Bill Table

**Date**: 2025-09-19
**Status**: No API Contracts Required

## Scope

This feature creates a database table schema only. Per the feature specification:
- No API endpoints required
- No REST/GraphQL contracts needed
- Focus on database schema definition

## Database Contract

### Table Schema Contract
The `bills` table must conform to the schema defined in `../data-model.md`:

- ✅ 14 fields as specified in requirements
- ✅ Proper PostgreSQL data types
- ✅ SERIAL primary key for auto-increment
- ✅ NUMERIC precision for financial calculations
- ✅ TEXT fields for Vietnamese text support

### Migration Contract
- ✅ Forward migration creates table successfully
- ✅ Rollback migration removes table safely
- ✅ SQLx compile-time validation passes
- ✅ No data loss on schema changes

## Validation Approach

Since no API contracts are required, validation focuses on:

1. **Schema Validation**: SQLx compile-time checks
2. **Migration Testing**: Forward and rollback operations
3. **Data Type Verification**: PostgreSQL constraint enforcement
4. **Query Compatibility**: Rust struct mapping validation

This contracts directory remains minimal as no external API surface is exposed for this database-only feature.