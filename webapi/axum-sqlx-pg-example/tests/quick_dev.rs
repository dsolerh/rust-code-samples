use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8091")?;

    // hc.do_get("/hello").await?.print().await?;
    // hc.do_get("/hello_query?name=Daniel").await?.print().await?;
    // hc.do_get("/hello_path/Daniel").await?.print().await?;
    // hc.do_get("/content/Cargo.toml").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome",
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
