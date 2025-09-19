use backend::config::ConnectionPool;
use backend::services::bill_service::BillService;
use backend::models::CreateBill;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment
    dotenvy::dotenv().ok();

    println!("Testing Bill struct mapping with sample queries...");

    // Create connection pool
    let pool = ConnectionPool::from_env().await?;
    let bill_service = BillService::new(pool.pool().clone());

    // Test 1: Get bills count
    println!("\n1. Testing bills count query...");
    match bill_service.get_bills_count().await {
        Ok(count) => println!("   Bills count: {}", count),
        Err(e) => println!("   Error: {:?}", e),
    }

    // Test 2: Get all bills
    println!("\n2. Testing get all bills...");
    match bill_service.get_all_bills().await {
        Ok(bills) => {
            println!("   Found {} bills:", bills.len());
            for bill in bills.iter().take(2) {
                println!("   - Bill ID {}: {}",
                    bill.id,
                    bill.invoice_no.as_ref().unwrap_or(&"No invoice".to_string())
                );
            }
        },
        Err(e) => println!("   Error: {:?}", e),
    }

    // Test 3: Get specific bill by ID
    println!("\n3. Testing get bill by ID (ID=1)...");
    match bill_service.get_bill_by_id(1).await {
        Ok(Some(bill)) => {
            println!("   Found bill: {}", bill.invoice_no.as_ref().unwrap_or(&"No invoice".to_string()));
            println!("   Seller: {}", bill.seller_name.as_ref().unwrap_or(&"No seller".to_string()));
            println!("   Amount: {:?}", bill.total_amount);
            println!("   Date: {:?}", bill.issued_date);
        },
        Ok(None) => println!("   Bill with ID 1 not found"),
        Err(e) => println!("   Error: {:?}", e),
    }

    // Test 4: Create a new bill with Rust types
    println!("\n4. Testing create bill with Rust struct mapping...");
    let create_bill = CreateBill {
        form_no: Some("Test Form".to_string()),
        serial_no: Some("TEST/001".to_string()),
        invoice_no: Some("TEST-2024-001".to_string()),
        issued_date: Some(NaiveDate::from_ymd_opt(2024, 12, 20).unwrap()),
        seller_name: Some("Test Company Ltd".to_string()),
        seller_tax_code: Some("1234567890".to_string()),
        item_name: Some("Test Item".to_string()),
        unit: Some("Piece".to_string()),
        quantity: Some(Decimal::from_str("5.00").unwrap()),
        unit_price: Some(Decimal::from_str("100000.50").unwrap()),
        total_amount: Some(Decimal::from_str("500002.50").unwrap()),
        vat_rate: Some(Decimal::from_str("10.00").unwrap()),
        vat_amount: Some(Decimal::from_str("50000.25").unwrap()),
    };

    match bill_service.create_bill(create_bill).await {
        Ok(new_bill) => {
            println!("   Created bill with ID: {}", new_bill.id);
            println!("   Invoice: {}", new_bill.invoice_no.as_ref().unwrap());
            println!("   Total: {:?}", new_bill.total_amount);
        },
        Err(e) => println!("   Error creating bill: {:?}", e),
    }

    // Test 5: Search bills by pattern
    println!("\n5. Testing search bills by invoice pattern...");
    match bill_service.search_bills_by_invoice("2024").await {
        Ok(bills) => {
            println!("   Found {} bills matching '2024':", bills.len());
            for bill in bills.iter().take(3) {
                println!("   - {}: {}",
                    bill.id,
                    bill.invoice_no.as_ref().unwrap_or(&"No invoice".to_string())
                );
            }
        },
        Err(e) => println!("   Error: {:?}", e),
    }

    println!("\nBill struct mapping test completed successfully!");
    Ok(())
}