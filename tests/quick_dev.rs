#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let local = httpc_test::new_client("http://localhost:8000")?;
    local.do_get("/hello").await?.print().await?;

    Ok(())
}
