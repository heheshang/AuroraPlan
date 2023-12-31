use bigdecimal::BigDecimal;
pub struct Model {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub str_prop_1: Option<String>,
    pub str_prop_2: Option<String>,
    pub str_prop_3: Option<String>,
    pub int_prop_1: Option<i32>,
    pub int_prop_2: Option<i32>,
    pub long_prop_1: Option<i64>,
    pub long_prop_2: Option<i64>,
    pub dec_prop_1: BigDecimal,
    pub dec_prop_2: BigDecimal,
    pub bool_prop_1: Option<bool>,
    pub bool_prop_2: Option<bool>,
}
