# Research: Backend Database Connection

## SQLx PostgreSQL Connection Patterns

**Decision**: Use SQLx PgPool with connection pooling and compile-time query validation

**Rationale**:
- SQLx provides async PostgreSQL drivers optimized for Tokio runtime
- Compile-time query validation prevents SQL errors at runtime
- PgPool automatically manages connections and handles concurrent requests
- Native Rust integration without heavyweight ORM overhead

**Alternatives considered**:
- Diesel ORM: Rejected due to constitution requirement for SQLx
- Raw tokio-postgres: Rejected due to lack of compile-time validation
- SeaORM: Rejected due to constitution requirement for SQLx only

## Axum + SQLx Connection Pooling Best Practices

**Decision**: Initialize PgPool at application startup and share via Axum State

**Rationale**:
- Axum State provides dependency injection for shared resources
- Connection pool should be created once and reused across requests
- Early initialization allows for graceful failure on database unavailability
- Shared state pattern is idiomatic in Axum applications

**Alternatives considered**:
- Lazy static connection: Rejected due to difficulty in error handling
- Per-request connections: Rejected due to performance overhead
- Global singleton: Rejected in favor of Axum's built-in state management

## Environment Configuration for Database URLs

**Decision**: Use dotenvy for .env file loading with DATABASE_URL environment variable

**Rationale**:
- dotenvy is already included in project dependencies
- DATABASE_URL is the standard convention for database connection strings
- Environment variables provide secure credential management
- Supports different environments (dev, staging, prod) without code changes

**Alternatives considered**:
- Config files (TOML/JSON): Rejected due to credential security concerns
- Command line arguments: Rejected due to credential exposure in process lists
- Hardcoded values: Rejected due to security and flexibility concerns

## Connection Pool Configuration

**Decision**: Configure pool with appropriate limits for bill_ocr database workload

**Rationale**:
- Default SQLx pool settings (max 10 connections) suitable for initial implementation
- Pool timeout and connection lifetime can be tuned based on usage patterns
- Proper pool sizing prevents connection exhaustion and database overload

**Implementation Details**:
- Max connections: 10 (default, can be configured via environment)
- Connection timeout: 30 seconds
- Idle timeout: 10 minutes
- Max lifetime: 30 minutes

## Database Connection Validation

**Decision**: Implement health check endpoint to verify database connectivity

**Rationale**:
- Health checks enable monitoring and debugging of database connectivity
- Essential for deployment validation and operational monitoring
- Provides immediate feedback on database configuration issues

**Implementation approach**:
- Simple SELECT 1 query to verify connection
- HTTP endpoint returning 200 OK or 503 Service Unavailable
- Include connection pool status in health check response