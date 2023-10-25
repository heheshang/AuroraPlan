/**
 * http check condition
 */
#[allow(non_camel_case_types)]
pub enum HttpCheckCondition {
    /**
     * 0 status_code_default:200
     * 1 status_code_custom
     * 2 body_contains
     * 3 body_not_contains
     */
    STATUS_CODE_DEFAULT,
    STATUS_CODE_CUSTOM,
    BODY_CONTAINS,
    BODY_NOT_CONTAINS,
}
