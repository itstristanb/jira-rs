/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// StatusUpdateRequest : The list of statuses that will be updated.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusUpdateRequest {
    /// The list of statuses that will be updated.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<crate::models::StatusUpdate>>,
}

impl StatusUpdateRequest {
    /// The list of statuses that will be updated.
    pub fn new() -> StatusUpdateRequest {
        StatusUpdateRequest {
            statuses: None,
        }
    }
}


