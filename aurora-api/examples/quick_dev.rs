use anyhow::Result;
// use aurora_config::api_config::Settings;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // let settings = Settings::new()?;
    // let host = settings.server.host;
    // let port = settings.server.port;
    let url = format!("http://{}:{}", "127.0.0.1", "54321");

    let hc = httpc_test::new_client(url)?;

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
