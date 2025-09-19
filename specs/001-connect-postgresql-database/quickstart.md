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
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15.7"
serde = { version = "1.0.225", features = ["derive"] }
serde_json = "1.0.145"
sqlx = { version = "0.8.6", features = ["runtime-tokio", "postgres", "macros", "chrono"] }
thiserror = "1.0"
tokio = { version = "1.47.1", features = ["full"] }
tower = { version = "0.5.2", features = ["timeout"] }
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1.11.0", features = ["v4"] }
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
# Check basic health endpoint
curl http://localhost:3000/health

# Expected response (healthy):
{
  "status": "healthy",
  "database_accessible": true,
  "pool_size": 1,
  "timestamp": "2025-09-19T12:00:00Z"
}

# Check detailed health endpoint
curl http://localhost:3000/health/detail

# Expected response (detailed):
{
  "status": "healthy",
  "database_accessible": true,
  "pool_size": 1,
  "timestamp": "2025-09-19T12:00:00Z",
  "max_connections": 10,
  "idle_connections": 1,
  "configuration": {
    "connection_timeout_seconds": 30,
    "idle_timeout_seconds": 600,
    "max_lifetime_seconds": 1800
  }
}
```

## Testing Connection

### Manual Testing
```bash
# Test basic health check
curl -X GET http://localhost:3000/health

# Test detailed health check
curl -X GET http://localhost:3000/health/detail

# Test invalid endpoint (should return 404)
curl -X GET http://localhost:3000/invalid
```

### Expected Behavior

**✅ Success Case**:
- Application starts without errors with comprehensive startup validation
- Health endpoint returns 200 OK with `"status": "healthy"`
- Detailed health endpoint returns 200 OK with full pool status
- Database connection pool is initialized with retry logic
- Startup logs show successful PostgreSQL version and database name verification

**❌ Failure Cases**:
- Application fails to start if DATABASE_URL is invalid
- Health endpoint returns 503 if database is unreachable
- Detailed health endpoint returns unhealthy status with database_accessible: false
- Error logs show specific connection issues and troubleshooting steps
- Invalid endpoints return 404 with structured error response

## Project Structure After Implementation

```
backend/
├── src/
│   ├── main.rs              # Application entry point with startup validation
│   ├── config/
│   │   ├── mod.rs          # Config module declarations and ConnectionPool
│   │   └── database.rs      # DatabaseConfig, DatabaseError types
│   ├── services/
│   │   └── health.rs        # HealthService, HealthStatus, DetailedHealthStatus
│   └── api/
│       ├── mod.rs          # API module with error handling and middleware
│       └── health.rs       # Health check routes (/health, /health/detail)
├── Cargo.toml               # Dependencies with chrono, uuid, tracing, etc.
└── .env                     # Environment variables (DATABASE_URL, etc.)
```

## Startup Validation Features

The implemented backend includes comprehensive startup validation:

1. **Database Connection Retry Logic**: Attempts connection with configurable retry count and delay
2. **PostgreSQL Version Verification**: Queries and logs the PostgreSQL version during startup
3. **Database Name Validation**: Confirms connection to the expected 'bill_ocr' database
4. **Connection Pool Status Logging**: Reports active and idle connection counts
5. **Graceful Error Handling**: Provides detailed error messages and troubleshooting guidance

## Troubleshooting

**Connection fails with "connection refused"**:
- Verify PostgreSQL is running on the specified host and port
- Check DATABASE_URL format: `postgresql://username:password@host:port/bill_ocr`
- Ensure the bill_ocr database exists and is accessible

**Permission denied errors**:
- Verify username/password credentials in DATABASE_URL
- Check that the user has appropriate privileges on the bill_ocr database
- Ensure the PostgreSQL user can create connections

**Pool exhaustion or timeout**:
- Monitor connection usage with `/health/detail` endpoint
- Adjust DB_MAX_CONNECTIONS environment variable if needed
- Check for connection leaks in application logs
- Use the detailed health endpoint to monitor pool utilization

**Application startup failure**:
- Check the detailed error logs for specific database validation failures
- Verify all required environment variables are set
- Ensure the database server accepts connections from your application host

## Available Endpoints

The backend provides the following REST API endpoints:

- **GET /health**: Basic health check returning 200 (healthy) or 503 (unhealthy)
- **GET /health/detail**: Detailed health information including pool status and configuration
- **All other routes**: Return structured 404 error responses

## Logging and Monitoring

The application includes comprehensive logging:
- **Request/Response Logging**: All HTTP requests with timing, status codes, and client info
- **Database Validation Logging**: Startup validation with PostgreSQL version and database verification
- **Error Logging**: Detailed error messages with appropriate log levels (info, warn, error)
- **Request IDs**: Unique UUID tracking for request correlation

## Next Steps

After successful database connection validation:
1. Implement OCR bill processing endpoints and database queries
2. Add database migration management for schema changes
3. Extend monitoring capabilities with metrics collection
4. Implement authentication and authorization layers