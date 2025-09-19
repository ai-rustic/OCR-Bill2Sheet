# Data Model: Backend Database Connection

## Core Entities

### DatabaseConfig
Configuration structure for database connection parameters.

**Fields**:
- `database_url: String` - PostgreSQL connection URL
- `max_connections: u32` - Maximum number of connections in pool (default: 10)
- `connection_timeout: Duration` - Timeout for acquiring connections (default: 30s)
- `idle_timeout: Option<Duration>` - Idle connection timeout (default: 10min)
- `max_lifetime: Option<Duration>` - Maximum connection lifetime (default: 30min)

**Validation Rules**:
- `database_url` must be valid PostgreSQL URL format
- `max_connections` must be > 0 and <= 100
- Timeout values must be positive

**State Transitions**: N/A (configuration is immutable after creation)

### ConnectionPool
Wrapper around SQLx PgPool for database connection management.

**Fields**:
- `pool: PgPool` - SQLx PostgreSQL connection pool
- `config: DatabaseConfig` - Configuration used to create pool

**Validation Rules**:
- Pool must be successfully created before use
- Connection must be testable via health check

**State Transitions**:
```
Uninitialized → Creating → Ready
                ↓
              Failed
```

### HealthStatus
Database connectivity health check response.

**Fields**:
- `status: String` - "healthy" or "unhealthy"
- `database_accessible: bool` - Can execute queries
- `pool_size: u32` - Current number of connections in pool
- `timestamp: DateTime<Utc>` - When health check was performed

**Validation Rules**:
- `status` must be "healthy" when `database_accessible` is true
- `pool_size` should be <= `max_connections` from config

## Entity Relationships

```
DatabaseConfig ---creates---> ConnectionPool
ConnectionPool ---generates--> HealthStatus
```

## Error Types

### DatabaseError
Custom error types for database operations.

**Variants**:
- `ConfigurationError(String)` - Invalid configuration
- `ConnectionError(sqlx::Error)` - Connection establishment failed
- `PoolError(String)` - Connection pool issues
- `HealthCheckError(sqlx::Error)` - Health check query failed

## Environment Variables

### Required
- `DATABASE_URL` - PostgreSQL connection string
  - Format: `postgresql://username:password@host:port/database_name`
  - Example: `postgresql://user:pass@localhost:5432/bill_ocr`

### Optional
- `DB_MAX_CONNECTIONS` - Override default max connections (default: 10)
- `DB_CONNECTION_TIMEOUT` - Override connection timeout in seconds (default: 30)
- `DB_IDLE_TIMEOUT` - Override idle timeout in seconds (default: 600)
- `DB_MAX_LIFETIME` - Override max lifetime in seconds (default: 1800)