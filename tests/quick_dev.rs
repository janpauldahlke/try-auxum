#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let local = httpc_test::new_client("http://localhost:8000")?;

    // Region : Hello
    local.do_get("/hello?name=jan").await?.print().await?;
    local.do_get("/hello2/frank").await?.print().await?;
    //serve from file or web dir
    local
        .do_get("/exposed_web/index.html")
        .await?
        .print()
        .await?;
    // endregion : Hello

    // Region : Login
    // is correct login
    local
        .do_post("/api/login", json!({"username": "jan", "pwd": "123"}))
        .await?
        .print()
        .await?;
    // is incorrect login
    local
        .do_post("/api/login", json!({"username": "paul", "pwd": "456"}))
        .await?
        .print()
        .await?;

    // endregion : Login

    Ok(())
}
