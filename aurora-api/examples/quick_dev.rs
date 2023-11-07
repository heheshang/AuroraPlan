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
    hc.cookie_store();
    let user_info = hc.do_get("/aurora/get_user_info").await?;
    user_info.print().await?;
    define_user_count().await?;

    Ok(())
}
// curl 'http://pt003:12345/dolphinscheduler/projects/analysis/define-user-count?projectCode=0' \
//   -H 'Accept: application/json, text/plain, */*' \
//   -H 'Accept-Language: zh-CN,zh;q=0.9' \
//   -H 'Cache-Control: no-cache' \
//   -H 'Connection: keep-alive' \
//   -H 'Cookie: sessionId=de87d07b-134f-4b6c-822d-30172e96c1b6; _ga=GA1.1.1260917633.1679362249; csrftoken=cau9yGs64fECWvFjv26ilGb23IG3PNOnCExY5mVIuQtGE4IhuP9DMtpcazyBFw71; language=zh_CN; session=.eJwljztuAzEMRO-iegtSoijRl1nwJySIEQO7dhXk7lkjmGaa9zDzU_Z15PlRbs_jlVvZP6PcihD36jgdhq1gnS7DyAGiQRKvmebc15orq5q6rrQYFPUK98AqA7NpQ3Jk5atT6DSAJm8rDwTs5pMiLCdhY5WqIkHZvBKVrfh5rP35-Mrva8-svceIAQTu9tYxibUOiMpjsAgsE8CLuz9c73kxF7iV15nH_yUsv3-u1UMH.ZTCT8Q.wL9PVkeZK9l0KBAuSwpdk088fnE' \
//   -H 'Pragma: no-cache' \
//   -H 'Referer: http://pt003:12345/dolphinscheduler/ui/home' \
//   -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
//   -H 'language: zh_CN' \
//   -H 'sessionId: de87d07b-134f-4b6c-822d-30172e96c1b6' \
//   --compressed \
//   --insecure:w

async fn define_user_count() -> Result<()> {
    let url = format!("http://{}:{}", "127.0.0.1", "54321");

    let hc = httpc_test::new_client(url)?;
    let req_login = hc.do_post(
        "/aurora/login",
        json!({
            "userName": "admin",
            "userPassword": "dolphinscheduler123"
        }),
    );
    req_login.await?.print().await?;
    hc.cookie_store();
    hc.do_get("/aurora/projects/analysis/define-user-count?projectCode=0")
        .await?
        .print()
        .await?;
    Ok(())
}

// curl 'http://pt003:12345/dolphinscheduler/projects/analysis/task-state-count?startDate=2023-11-06%2000%3A00%3A00&endDate=2023-11-06%2014%3A46%3A54&projectCode=0' \
//   -H 'Accept: application/json, text/plain, */*' \
//   -H 'Accept-Language: zh-CN,zh;q=0.9' \
//   -H 'Cache-Control: no-cache' \
//   -H 'Connection: keep-alive' \
//   -H 'Cookie: sessionId=de87d07b-134f-4b6c-822d-30172e96c1b6; _ga=GA1.1.1260917633.1679362249; csrftoken=cau9yGs64fECWvFjv26ilGb23IG3PNOnCExY5mVIuQtGE4IhuP9DMtpcazyBFw71; language=zh_CN; session=.eJwljztuAzEMRO-iegtSoijRl1nwJySIEQO7dhXk7lkjmGaa9zDzU_Z15PlRbs_jlVvZP6PcihD36jgdhq1gnS7DyAGiQRKvmebc15orq5q6rrQYFPUK98AqA7NpQ3Jk5atT6DSAJm8rDwTs5pMiLCdhY5WqIkHZvBKVrfh5rP35-Mrva8-svceIAQTu9tYxibUOiMpjsAgsE8CLuz9c73kxF7iV15nH_yUsv3-u1UMH.ZTCT8Q.wL9PVkeZK9l0KBAuSwpdk088fnE' \
//   -H 'Pragma: no-cache' \
//   -H 'Referer: http://pt003:12345/dolphinscheduler/ui/home' \
//   -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
//   -H 'language: zh_CN' \
//   -H 'sessionId: de87d07b-134f-4b6c-822d-30172e96c1b6' \
//   --compressed \
//   --insecure

// curl 'http://pt003:12345/dolphinscheduler/projects/analysis/process-state-count?startDate=2023-11-06%2000%3A00%3A00&endDate=2023-11-06%2014%3A46%3A54&projectCode=0' \
//   -H 'Accept: application/json, text/plain, */*' \
//   -H 'Accept-Language: zh-CN,zh;q=0.9' \
//   -H 'Cache-Control: no-cache' \
//   -H 'Connection: keep-alive' \
//   -H 'Cookie: sessionId=de87d07b-134f-4b6c-822d-30172e96c1b6; _ga=GA1.1.1260917633.1679362249; csrftoken=cau9yGs64fECWvFjv26ilGb23IG3PNOnCExY5mVIuQtGE4IhuP9DMtpcazyBFw71; language=zh_CN; session=.eJwljztuAzEMRO-iegtSoijRl1nwJySIEQO7dhXk7lkjmGaa9zDzU_Z15PlRbs_jlVvZP6PcihD36jgdhq1gnS7DyAGiQRKvmebc15orq5q6rrQYFPUK98AqA7NpQ3Jk5atT6DSAJm8rDwTs5pMiLCdhY5WqIkHZvBKVrfh5rP35-Mrva8-svceIAQTu9tYxibUOiMpjsAgsE8CLuz9c73kxF7iV15nH_yUsv3-u1UMH.ZTCT8Q.wL9PVkeZK9l0KBAuSwpdk088fnE' \
//   -H 'Pragma: no-cache' \
//   -H 'Referer: http://pt003:12345/dolphinscheduler/ui/home' \
//   -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
//   -H 'language: zh_CN' \
//   -H 'sessionId: de87d07b-134f-4b6c-822d-30172e96c1b6' \
//   --compressed \
//   --insecure
