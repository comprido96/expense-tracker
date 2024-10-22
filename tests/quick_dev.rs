#[allow(unused)]

use anyhow::Result;
use serde_json::json;
use chrono::prelude::*;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "fede",
            "pwd": "fede",
        })
    );

    req_login.await?.print().await?;

    let req_create_expense = hc.do_post(
        "/api/expenses",
        json!({
            "payment_date": Utc.with_ymd_and_hms(2024, 11, 5, 0, 0, 0).unwrap(),
            "description": "affitto mensile".to_string(),
            "amount": 465,
            "is_recurrent": true,
        })
    );

    req_create_expense.await?.print().await?;

    Ok(())
}