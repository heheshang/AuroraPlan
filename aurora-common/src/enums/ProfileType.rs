pub struct ProfileType {
    pub H2: String,
    pub MYSQL: String,
    pub POSTGRESQL: String,
    pub DATASOURCE_PROFILE: Vec<String>,
}

impl ProfileType {
    pub fn new() -> ProfileType {
        ProfileType {
            H2: String::from("h2"),
            MYSQL: String::from("mysql"),
            POSTGRESQL: String::from("postgresql"),
            DATASOURCE_PROFILE: vec![
                String::from("h2"),
                String::from("mysql"),
                String::from("postgresql"),
            ],
        }
    }
}

impl Default for ProfileType {
    fn default() -> Self {
        Self::new()
    }
}
