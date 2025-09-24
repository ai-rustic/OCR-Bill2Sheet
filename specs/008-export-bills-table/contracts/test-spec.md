# Contract Tests: Bills Export API

**Phase**: 1 - Design & Contracts
**Date**: 2025-09-24
**Feature**: Export Bills Table

## Test Structure

### Test Framework
- **Framework**: Rust `tokio-test` + `axum-test`
- **Database**: PostgreSQL test database with sample data
- **Setup**: Test fixtures with known bill records
- **Cleanup**: Transaction rollback after each test

## Contract Test Cases

### TC-001: CSV Export Success
**Description**: Verify CSV export with valid format parameter returns proper file

```rust
#[tokio::test]
async fn test_csv_export_success() {
    // Given: Database with sample bills
    let app = create_test_app().await;
    let bills = create_sample_bills(&app.db_pool).await;

    // When: Request CSV export
    let response = app
        .get("/api/bills/export?format=csv")
        .await;

    // Then: Verify response
    assert_eq!(response.status(), StatusCode::OK);

    // Verify headers
    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "text/csv; charset=utf-8"
    );
    assert!(headers.get("content-disposition").unwrap()
        .to_str().unwrap()
        .starts_with("attachment; filename=\"bills_export_"));
    assert_eq!(
        headers.get("cache-control").unwrap(),
        "no-cache, no-store, must-revalidate"
    );

    // Verify content structure
    let body = response.text().await;
    assert!(body.starts_with('\u{feff}')); // UTF-8 BOM
    assert!(body.contains("ID,Số tờ khai / Form No")); // Headers
    assert!(body.contains("1,Mẫu 01-GTKT")); // Sample data
}
```

### TC-002: XLSX Export Success
**Description**: Verify XLSX export with valid format parameter returns proper file

```rust
#[tokio::test]
async fn test_xlsx_export_success() {
    // Given: Database with sample bills
    let app = create_test_app().await;
    let bills = create_sample_bills(&app.db_pool).await;

    // When: Request XLSX export
    let response = app
        .get("/api/bills/export?format=xlsx")
        .await;

    // Then: Verify response
    assert_eq!(response.status(), StatusCode::OK);

    // Verify headers
    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    );
    assert!(headers.get("content-disposition").unwrap()
        .to_str().unwrap()
        .starts_with("attachment; filename=\"bills_export_"));

    // Verify content is valid XLSX (binary check)
    let body = response.bytes().await;
    assert!(body.len() > 0);
    // XLSX files start with PK header
    assert_eq!(&body[0..2], b"PK");
}
```

### TC-003: Invalid Format Parameter
**Description**: Verify error response for unsupported format parameter

```rust
#[tokio::test]
async fn test_invalid_format_parameter() {
    // Given: Test app
    let app = create_test_app().await;

    // When: Request with invalid format
    let response = app
        .get("/api/bills/export?format=pdf")
        .await;

    // Then: Verify error response
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let error: serde_json::Value = response.json().await;
    assert_eq!(error["error"], "Invalid format parameter");
    assert_eq!(error["code"], "INVALID_FORMAT");
    assert!(error["message"].as_str().unwrap()
        .contains("Supported formats: csv, xlsx"));
}
```

### TC-004: Missing Format Parameter
**Description**: Verify error response when format parameter is missing

```rust
#[tokio::test]
async fn test_missing_format_parameter() {
    // Given: Test app
    let app = create_test_app().await;

    // When: Request without format parameter
    let response = app
        .get("/api/bills/export")
        .await;

    // Then: Verify error response
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let error: serde_json::Value = response.json().await;
    assert_eq!(error["error"], "Missing required parameter");
    assert_eq!(error["code"], "MISSING_PARAMETER");
    assert!(error["message"].as_str().unwrap()
        .contains("Format parameter is required"));
}
```

### TC-005: Empty Database Export
**Description**: Verify export behavior when database contains no bills

```rust
#[tokio::test]
async fn test_empty_database_csv_export() {
    // Given: Empty database
    let app = create_test_app().await;
    clear_all_bills(&app.db_pool).await;

    // When: Request CSV export
    let response = app
        .get("/api/bills/export?format=csv")
        .await;

    // Then: Verify headers-only response
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await;
    assert!(body.starts_with('\u{feff}')); // UTF-8 BOM
    assert!(body.contains("ID,Số tờ khai / Form No")); // Headers present

    // Count lines - should be header + empty line
    let lines: Vec<&str> = body.lines().collect();
    assert_eq!(lines.len(), 1); // Only header line
}

#[tokio::test]
async fn test_empty_database_xlsx_export() {
    // Given: Empty database
    let app = create_test_app().await;
    clear_all_bills(&app.db_pool).await;

    // When: Request XLSX export
    let response = app
        .get("/api/bills/export?format=xlsx")
        .await;

    // Then: Verify valid but empty XLSX
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.bytes().await;
    assert!(body.len() > 0); // Should still have file structure
    assert_eq!(&body[0..2], b"PK"); // Valid XLSX header
}
```

### TC-006: Vietnamese Text Encoding
**Description**: Verify proper handling of Vietnamese characters in export

```rust
#[tokio::test]
async fn test_vietnamese_text_encoding() {
    // Given: Bills with Vietnamese text
    let app = create_test_app().await;
    let vietnamese_bill = Bill {
        id: 1,
        form_no: Some("Mẫu 01-GTKT".to_string()),
        invoice_no: Some("HĐ-2024-001".to_string()),
        seller_name: Some("Công ty TNHH Xuất Nhập Khẩu Việt Nam".to_string()),
        buyer_name: Some("Khách hàng Nguyễn Văn Anh".to_string()),
        // ... other fields
    };
    insert_bill(&app.db_pool, vietnamese_bill).await;

    // When: Request CSV export
    let response = app
        .get("/api/bills/export?format=csv")
        .await;

    // Then: Verify Vietnamese text preservation
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await;
    assert!(body.contains("Mẫu 01-GTKT"));
    assert!(body.contains("HĐ-2024-001"));
    assert!(body.contains("Công ty TNHH Xuất Nhập Khẩu Việt Nam"));
    assert!(body.contains("Khách hàng Nguyễn Văn Anh"));
}
```

### TC-007: Large Dataset Performance
**Description**: Verify streaming performance with large number of records

```rust
#[tokio::test]
async fn test_large_dataset_export() {
    // Given: Large number of bills (5000 records)
    let app = create_test_app().await;
    create_large_bill_dataset(&app.db_pool, 5000).await;

    // When: Request export
    let start_time = std::time::Instant::now();
    let response = app
        .get("/api/bills/export?format=csv")
        .await;
    let duration = start_time.elapsed();

    // Then: Verify reasonable performance
    assert_eq!(response.status(), StatusCode::OK);
    assert!(duration.as_secs() < 10); // Should complete within 10 seconds

    let body = response.text().await;
    let line_count = body.lines().count();
    assert_eq!(line_count, 5001); // Header + 5000 data lines
}
```

### TC-008: Concurrent Requests
**Description**: Verify system handles multiple concurrent export requests

```rust
#[tokio::test]
async fn test_concurrent_export_requests() {
    // Given: Database with bills
    let app = create_test_app().await;
    create_sample_bills(&app.db_pool).await;

    // When: Send multiple concurrent requests
    let mut handles = vec![];
    for _ in 0..5 {
        let app_clone = app.clone();
        handles.push(tokio::spawn(async move {
            app_clone.get("/api/bills/export?format=csv").await
        }));
    }

    // Then: All requests should succeed
    let results = futures::future::join_all(handles).await;
    for result in results {
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

## Test Data Fixtures

### Sample Bills
```rust
async fn create_sample_bills(pool: &sqlx::PgPool) -> Vec<Bill> {
    vec![
        Bill {
            id: 1,
            form_no: Some("Mẫu 01-GTKT".to_string()),
            invoice_no: Some("INV-2024-001".to_string()),
            invoice_date: Some(chrono::NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()),
            seller_name: Some("Công ty ABC".to_string()),
            seller_tax_code: Some("0123456789".to_string()),
            total_amount: Some(rust_decimal::Decimal::from_str("1000000.50").unwrap()),
            tax_amount: Some(rust_decimal::Decimal::from_str("100000.05").unwrap()),
            vat_rate: Some(rust_decimal::Decimal::from_str("10.00").unwrap()),
            // ... other fields
        },
        // Additional sample records...
    ]
}
```

## Test Execution Strategy

### Test Environment
- **Database**: Isolated test database per test suite
- **Transactions**: Each test runs in rollback transaction
- **Cleanup**: Automatic cleanup after test completion
- **Parallelization**: Tests can run in parallel with isolated data

### Performance Benchmarks
- **Small dataset** (1-100 records): <100ms response time
- **Medium dataset** (100-1000 records): <1s response time
- **Large dataset** (1000-10000 records): <10s response time
- **Memory usage**: Constant regardless of dataset size (streaming)

### Test Categories
1. **Happy Path**: Valid requests with expected responses
2. **Error Handling**: Invalid inputs and error conditions
3. **Edge Cases**: Empty data, large datasets, special characters
4. **Performance**: Response time and memory usage
5. **Concurrency**: Multiple simultaneous requests