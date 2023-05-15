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

    // // is incorrect login
    local
        .do_post("/api/login", json!({"username": "paul", "pwd": "456"}))
        .await?
        .print()
        .await?;
    // endregion : Login

    // region cookies
    //naive loop
    // for n in 1..10 {
    //     println!("--> {:<12} - cookies - {n}", "LOOP");
    //     local.do_get("/api/cookies").await?.print().await?;
    // }
    // endregion cookies

    // region : Tickets
    //post a ticket
    let request_to_create_ticket = local
        .do_post(
            "/api/tickets",
            json!({
                "title": "test ticket",

            }),
        )
        .await?
        .print()
        .await?;

    //get a list of ticket
    local.do_get("/api/tickets").await?.print().await?;
    //delete a ticket
    local.do_delete("/api/tickets/1").await?.print().await?; // bewaere, there is not always a ticket with id 1

    // verfiy deletion // comment in/out
    local.do_get("/api/tickets").await?.print().await?;

    // endregion : Tickets

    Ok(())
}
