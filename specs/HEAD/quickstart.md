# Quickstart: Backend Database Connection

## Prerequisites
- Rust 1.75+ installed
- PostgreSQL database named `bill_ocr` accessible
- Environment variables configured

## Environment Setup

1. **Create .env file** in the backend directory:
```bash
cd backend
echo "DATABASE_URL=postgresql://username:password@localhost:5432/bill_ocr" > .env
```

2. **Optional configuration** (add to .env if needed):
```bash
# Optional: Override default connection pool settings
DB_MAX_CONNECTIONS=10
DB_CONNECTION_TIMEOUT=30
DB_IDLE_TIMEOUT=600
DB_MAX_LIFETIME=1800
```

## Quick Start Steps

1. **Verify dependencies** in backend/Cargo.toml:
```toml
[dependencies]
axum = "0.8.4"
sqlx = { version = "0.8.6", features = ["runtime-tokio", "postgres", "macros"] }
tokio = { version = "1.47.1", features = ["full"] }
dotenvy = "0.15.7"
```

2. **Build the project**:
```bash
cd backend
cargo build
```

3. **Run the application**:
```bash
cd backend
cargo run
```

4. **Verify database connection**:
```bash
# Check health endpoint
curl http://localhost:3000/health

# Expected response:
{
  "status": "healthy",
  "database_accessible": true,
  "pool_size": 1,
  "timestamp": "2025-09-19T12:00:00Z"
}
```

## Testing Connection

### Manual Testing
```bash
# Test health check
curl -X GET http://localhost:3000/health

# Test detailed health
curl -X GET http://localhost:3000/health/detail

# Test configuration endpoint
curl -X GET http://localhost:3000/config/database
```

### Expected Behavior

**✅ Success Case**:
- Application starts without errors
- Health endpoint returns 200 OK with `"status": "healthy"`
- Database connection pool is initialized

**❌ Failure Cases**:
- Application fails to start if DATABASE_URL is invalid
- Health endpoint returns 503 if database is unreachable
- Error logs show specific connection issues

## Project Structure After Implementation

```
backend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config/
│   │   └── database.rs      # Database configuration
│   ├── services/
│   │   └── health.rs        # Health check service
│   └── api/
│       ├── mod.rs          # API module declarations
│       ├── health.rs       # Health check routes
│       └── config.rs       # Configuration routes
├── Cargo.toml
├── .env                     # Environment variables
└── tests/
    ├── integration/
    │   └── database_test.rs # Database connectivity tests
    └── contract/
        └── health_test.rs   # API contract tests
```

## Integration Test Example

```rust
#[tokio::test]
async fn test_database_connection() {
    // Load environment
    dotenvy::dotenv().ok();

    // Initialize database connection
    let config = DatabaseConfig::from_env().unwrap();
    let pool = ConnectionPool::new(config).await.unwrap();

    // Test connection
    let health = pool.health_check().await.unwrap();
    assert!(health.database_accessible);
    assert_eq!(health.status, "healthy");
}
```

## Troubleshooting

**Connection fails with "connection refused"**:
- Verify PostgreSQL is running
- Check host and port in DATABASE_URL
- Ensure bill_ocr database exists

**Permission denied errors**:
- Verify username/password in DATABASE_URL
- Check user has access to bill_ocr database

**Pool exhaustion**:
- Monitor connection usage patterns
- Adjust DB_MAX_CONNECTIONS if needed
- Check for connection leaks in application code

## Next Steps

After successful database connection:
1. Implement specific database queries for OCR bill processing
2. Add database migration management
3. Implement proper error handling and logging
4. Add monitoring and metrics collection