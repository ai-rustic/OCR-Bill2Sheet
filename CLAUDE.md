# OCR_Bill2Sheet Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-09-19

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

## Current Feature: Database Connection
- Connect backend to PostgreSQL bill_ocr database
- No table creation or model modifications required
- Use existing database schema
- Environment-driven configuration

<!-- MANUAL ADDITIONS END -->
