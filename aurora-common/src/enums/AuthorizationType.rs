#[allow(non_camel_case_types)]
/**
 * Authorization type
 */
#[derive(Clone, Debug)]
pub enum AuthorizationType {
    /**
     * 0 RESOURCE_FILE_ID;
     * 0 RESOURCE_FILE_NAME;
     * 1 UDF_FILE;
     * 1 DATASOURCE;
     * 2 UDF;
     */
    RESOURCE_FILE_ID,
    #[allow(non_camel_case_types)]
    RESOURCE_FILE_NAME,
    #[allow(non_camel_case_types)]
    UDF_FILE,
    DATASOURCE,
    UDF,
}

#[derive(Clone, Debug)]
pub struct Message {
    code: i32,
    descp: String,
}

impl Message {
    fn new(code: i32, descp: String) -> Message {
        Message { code, descp }
    }
}

impl AuthorizationType {
    pub fn get_message(&self) -> Message {
        match self {
            AuthorizationType::RESOURCE_FILE_ID => Message::new(0, "resource file id".to_string()),
            AuthorizationType::RESOURCE_FILE_NAME => Message::new(1, "resource file name".to_string()),
            AuthorizationType::UDF_FILE => Message::new(2, "udf file".to_string()),
            AuthorizationType::DATASOURCE => Message::new(3, "data source".to_string()),
            AuthorizationType::UDF => Message::new(4, "udf function".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<AuthorizationType> for Message {
    fn from(alert_status: AuthorizationType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for AuthorizationType {
    fn from(value: Message) -> Self {
        let a: AuthorizationType = value.into();
        a
    }
}
