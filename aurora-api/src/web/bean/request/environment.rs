use std::{cmp, fmt, marker::PhantomData};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::json;
use serde_with::serde_as;
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironment {
    pub name: String,
    pub config: String,
    pub description: Option<String>,
    // #[serde_with::serde_as(deserialize_as = "Vec<String>")]
    // #[serde_as(as = "Vec<String>")]
    pub worker_groups: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct EnvironmentListParams {
    pub pageSize: u64,
    pub pageNo: u64,
    pub searchVal: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEnvironment {
    pub code: i64,
    pub name: String,
    pub config: String,
    pub description: Option<String>,
    pub worker_groups: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEnvironment {
    pub environment_name: String,
}

#[test]
fn test_deserialize_worker_groups() {
    let env = CreateEnvironment {
        name: "test".to_string(),
        config: "test".to_string(),
        description: None,
        worker_groups: "[\"default\"]".to_string(),
    };
    let mut ss = env.worker_groups.clone();
    ss = ss
        .replace(['[', ']', '\"'], "")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", ss);
}
