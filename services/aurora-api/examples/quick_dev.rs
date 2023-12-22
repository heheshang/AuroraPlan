use anyhow::Result;

// use api/lib_conifg::api_config::Settings;
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
        "/api/aurora/login",
        json!({
            "userName": "admin",
            "userPassword": "dolphinscheduler123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/aurora/login",
        json!({
            "userName": "admin",
            "userPassword": "dolphinscheduler123"
        }),
    );
    req_login.await?.print().await?;
    hc.cookie_store();
    let user_info = hc.do_get("/api/aurora/get_user_info").await?;
    user_info.print().await?;
    define_user_count().await?;
    task_state_count().await?;
    process_state_count().await?;
    projects().await?;
    Ok(())
}

async fn define_user_count() -> Result<()> {
    let hc = login!();
    hc.do_get("/api/aurora/projects/analysis/define-user-count?projectCode=0")
        .await?
        .print()
        .await?;
    Ok(())
}
#[macro_export]
macro_rules! login {
    () => {{
        let url = format!("http://{}:{}", "127.0.0.1", "54321");

        let hc = httpc_test::new_client(url)?;
        let req_login = hc.do_post(
            "/api/aurora/login",
            json!({
                "userName": "admin",
                "userPassword": "dolphinscheduler123"
            }),
        );
        req_login.await?;
        hc.cookie_store();
        hc
        }};

}

async fn task_state_count() -> Result<()> {
    let hc = login!();
    hc.do_get("/api/aurora/projects/analysis/task-state-count?projectCode=0&startDate=2023-11-07&endDate=2023-11-07")
        .await?
        .print()
        .await?;
    Ok(())
}
//http://pt003:12345/dolphinscheduler/projects/analysis/process-state-count?startDate=2023-11-07%2000%3A00%3A00&endDate=2023-11-07%2009%3A04%3A42&projectCode=0
async fn process_state_count() -> Result<()> {
    let hc = login!();
    hc.do_get(
        "/api/aurora/projects/analysis/process-state-count?projectCode=0&startDate=2023-11-07&endDate=2023-11-00",
    )
    .await?
    .print()
    .await?;
    Ok(())
}
//http://pt003:12345/dolphinscheduler/projects?pageSize=10&pageNo=1&searchVal=

async fn projects() -> Result<()> {
    let hc = login!();
    hc.do_get("/api/aurora/projects?pageSize=10&pageNo=1&searchVal=")
        .await?
        .print()
        .await?;
    Ok(())
}
// curl 'http://localhost:5174/api/aurora/signOut' \
//  -X 'POST' \
//  -H 'Accept: application/json, text/plain, */*' \
//  -H 'Accept-Language: zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6' \
//  -H 'Connection: keep-alive' \
//  -H 'Content-Length: 0' \
//  -H 'Cookie: _xsrf=2|4ff02522|e82fd20fcc36095cdbbff42f01765d64|1696900613; username-localhost-12345="2|1:0|10:1696916069|24:username-localhost-12345|44:NDg3OWY2YWJhYmVmNDc4YWI0NDhmZmJiNTU5YjViNWQ=|3f6d74d804a47417afaa0319887a4d032eeefc6d0b553cac7a6bceba2fbb795f"; username-localhost-8888="2|1:0|10:1696916464|23:username-localhost-8888|44:YzFhOTU2N2FhN2YyNDY3ZGE2ODRmYmU4MzY0YzlhZmU=|1be5150595733d56393de480a7ce9162c9ad42545ae2042c6afc47cea8cae8de"; username-localhost-12346="2|1:0|10:1696920283|24:username-localhost-12346|44:MDZmMjI1NjRlNjQxNGFhYjkwMmVkY2E4N2M0ZTRkMjA=|e1b8c763e681eb08bc80677d604f418d2ab3ed2f6420e4a9210b53f92e7e8150"' \
//  -H 'Origin: http://localhost:5174' \
//  -H 'Referer: http://localhost:5174/home' \
//  -H 'Sec-Fetch-Dest: empty' \
//  -H 'Sec-Fetch-Mode: cors' \
//  -H 'Sec-Fetch-Site: same-origin' \
//  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.76' \
//  -H 'sec-ch-ua: "Chromium";v="118", "Microsoft Edge";v="118", "Not=A?Brand";v="99"' \
//  -H 'sec-ch-ua-mobile: ?0' \
//  -H 'sec-ch-ua-platform: "Windows"' \
//  -H 'sessionId;' \
////   --compressed
