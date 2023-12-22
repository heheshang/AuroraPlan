use lib_common::core_results::results::Result;

use aurora_api::cypt::security::get_authenticator;
#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let username = "admin".to_string();
    let password = "dolphinscheduler123".to_string();
    let extra = "127.0.0.1".to_string();
    let res = get_authenticator().authenticate(username, password, extra).await?;
    println!("res: {:?}", res);
    Ok(())
}
