#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Rust").await?.print().await?;
    hc.do_get("/hello2/Pathfinder").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!(
            {
               "username": "demo1",
               "pwd": "welcome"
            }
        ),
    );
    req_login.await?.print().await?;

    let req_login_wrong_pwd = hc.do_post(
        "/api/login",
        json!(
            {
               "username": "demo1",
               "pwd": "not welcome"
            }
        ),
    );
    req_login_wrong_pwd.await?.print().await?;

    let req_login_wrong_json = hc.do_post(
        "/api/login",
        json!(
            {
               "username": "demo1",
               "wrongField": "welcome"
            }
        ),
    );
    req_login_wrong_json.await?.print().await?;
    // pretty nice error handling, e.g. missing field in json
    // 422 Unprocessable Entity
    // Failed to deserialize the JSON body into the target type: missing field `pwd` at line 1 column 43


    // won't always call this route because of cli spam, but nice to see that tokio even generates
    // a proper content-type header: "text/x-rust"

    // hc.do_get("/src/main.rs").await?.print().await?;
    Ok(())
}
