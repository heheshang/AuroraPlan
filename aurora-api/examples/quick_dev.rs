use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:54321")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/aurora/login",
        json!({
            "userName": "admin",
            "userPassword": "dolphinscheduler123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/aurora/login",
        json!({
            "userName": "admin",
            "userPassword": "dolphinscheduler123"
        }),
    );
    req_login.await?.print().await?;
    let user_info = hc.do_get("/aurora/get_user_info").await?;
    user_info.print().await?;
    Ok(())
}
