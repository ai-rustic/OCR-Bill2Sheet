/ru# OCR_Bill2Sheet Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-09-20

## Active Technologies
- Rust 1.75+ (edition = "2024") + Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7 (HEAD)

## Project Structure
```
backend/
frontend/
tests/
```

## Commands
cargo test && cargo clippy

## Code Style
Rust 1.75+ (edition = "2024"): Follow standard conventions

## Recent Changes
- 002-create-a-bill: Bill table schema with 14 fields for Vietnamese invoices, SQLx migrations
- HEAD: Added Rust 1.75+ (edition = "2024") + Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7

<!-- MANUAL ADDITIONS START -->

## Constitutional Requirements

### Backend (Axum + SQLx)
- Use SQLx with compile-time query validation (`query!`, `query_as!`)
- Connection pooling required for PostgreSQL (bill_ocr database)
- Environment-based configuration via DATABASE_URL
- No ORM except SQLx - follow constitution strictly

### Frontend (Shadcn-First)
- ALL UI components must use Shadcn UI
- Verify components with `mcp` before implementation
- No custom UI components allowed
- Mobile-first responsive design

### Development Workflow
- **TDD is PROHIBITED** - Implementation-first approach
- Speed and prototype delivery prioritized
- No test scaffolding during development

## Current Feature: Bill Table Schema ✅ COMPLETED
- Bill table with 14 fields for Vietnamese invoice data
- SQLx migrations for schema management
- NUMERIC(18,2) precision for financial calculations
- TEXT fields for Vietnamese text support
- Compile-time query validation with SQLx macros

## Migration Patterns and Usage Examples

### Migration Commands
```bash
# Create new migration
cd backend && sqlx migrate add create_bill_table

# Run migrations
cd backend && sqlx migrate run

# Rollback migration
cd backend && sqlx migrate revert

# Check migration status
cd backend && sqlx migrate info
```

### Bill Service Usage Examples
```rust
use backend::config::ConnectionPool;
use backend::services::bill_service::BillService;
use backend::models::{Bill, CreateBill};

// Initialize service
let pool = ConnectionPool::from_env().await?;
let bill_service = BillService::new(pool.pool().clone());

// Get all bills
let bills = bill_service.get_all_bills().await?;

// Create new bill
let new_bill = CreateBill {
    form_no: Some("Mẫu 01-GTKT".to_string()),
    invoice_no: Some("INV-2024-001".to_string()),
    // ... other fields
};
let created = bill_service.create_bill(new_bill).await?;

// Search bills
let results = bill_service.search_bills_by_invoice("2024").await?;
```

### Validated Field Mappings
- PostgreSQL SERIAL → Rust i32
- PostgreSQL TEXT → Rust Option<String>
- PostgreSQL DATE → Rust Option<chrono::NaiveDate>
- PostgreSQL NUMERIC(18,2) → Rust Option<rust_decimal::Decimal>
- PostgreSQL NUMERIC(5,2) → Rust Option<rust_decimal::Decimal>

### Migration Safety Procedures
1. Always test rollback before deploying: `sqlx migrate revert`
2. Verify table structure: `psql $DATABASE_URL -c "\d bills"`
3. Test data integrity with edge cases
4. Use compile-time validation with SQLx macros
5. Validate Vietnamese text and financial precision

<!-- MANUAL ADDITIONS END -->


