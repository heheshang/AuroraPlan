use lib_proto::ds_user::DsUser;
use lib_proto::ds_user::DsUserPage;
use lib_proto::ds_user::ListDsUsersResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Debug, Serialize, Deserialize, Convert)]
#[convert(from = "DsUser")]
#[serde(rename_all = "camelCase")]
pub struct UserInfoRes {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    #[serde(with = "convert_user_type")]
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    /// google.lib_protobuf.Timestamp create_time=8
    pub create_time: Option<String>,
    /// optional google.lib_protobuf.Timestamp update_time=9;
    pub update_time: Option<String>,

    pub queue: Option<String>,
    pub state: Option<i32>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsUsersResponse")]
pub struct UserList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<UserInfoPage>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[convert(from = "DsUserPage")]
#[serde(rename_all = "camelCase")]
pub struct UserInfoPage {
    pub id: i32,
    pub user_name: Option<String>,
    #[serde(skip_serializing)]
    pub user_password: Option<String>,
    #[serde(with = "convert_user_type")]
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    /// google.lib_protobuf.Timestamp create_time=8
    pub create_time: Option<String>,
    /// optional google.lib_protobuf.Timestamp update_time=9;
    pub update_time: Option<String>,

    pub queue: Option<String>,
    pub state: Option<i32>,
    pub time_zone: Option<String>,
    pub tenant_code: Option<String>,
}
mod convert_user_type {
    use std::str::FromStr;

    use lib_common::enums::UserType::UserType;
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(user_type: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let user_type = UserType::of(&user_type.unwrap_or(0));
        let s = format!("{:?}", user_type);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let ss = UserType::from_str(&s);
        match ss {
            Ok(s) => Ok(Some(s.get_code())),
            Err(_) => Err(serde::de::Error::custom(format!("invalid user type: {}", s))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginInfoRes {
    pub session_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_serialize() {
        let user = UserInfoRes {
            id: 1,
            user_name: Some("test".to_string()),
            user_password: Some("test".to_string()),
            user_type: Some(0),
            email: Some("test".to_string()),
            phone: Some("test".to_string()),
            tenant_id: Some(1),
            create_time: Some("test".to_string()),
            update_time: Some("test".to_string()),
            queue: Some("test".to_string()),
            state: Some(1),
            time_zone: Some("test".to_string()),
        };
        let json = serde_json::to_string(&user).unwrap();
        println!("{}", json);
        let user: UserInfoRes = serde_json::from_str(&json).unwrap();
        assert_eq!(user.user_type, Some(0));
    }
}
