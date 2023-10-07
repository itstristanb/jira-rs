/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedResponse {
    #[serde(rename = "errorCollection", skip_serializing_if = "Option::is_none")]
    pub error_collection: Option<Box<crate::models::ErrorCollection>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "warningCollection", skip_serializing_if = "Option::is_none")]
    pub warning_collection: Option<Box<crate::models::WarningCollection>>,
}

impl NestedResponse {
    pub fn new() -> NestedResponse {
        NestedResponse {
            error_collection: None,
            status: None,
            warning_collection: None,
        }
    }
}

