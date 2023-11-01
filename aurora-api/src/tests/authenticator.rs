use aurora_common::core_results::results::Result;

use crate::cypt::security::get_authenticator;
#[tokio::test]
async fn authenticate() -> Result<()> {
    let username = "admin".to_string();
    let password = "aurora123".to_string();
    let extra = "127.0.0.1".to_string();
    let res = get_authenticator()
        .authenticate(username, password, extra)
        .await?;
    println!("res: {:?}", res);
    Ok(())
}
