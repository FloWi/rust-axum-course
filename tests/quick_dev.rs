#![allow(unused)] // For beginning only.

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Rust").await?.print().await?;
    hc.do_get("/hello2/Pathfinder").await?.print().await?;

    // won't use it, because of cli spam, but nice to see that tokio even generates
    // a proper content-type header: "text/x-rust"

    // hc.do_get("/src/main.rs").await?.print().await?;
    Ok(())
}

