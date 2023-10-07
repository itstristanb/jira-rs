/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PublishDraftWorkflowScheme : Details about the status mappings for publishing a draft workflow scheme.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishDraftWorkflowScheme {
    /// Mappings of statuses to new statuses for issue types.
    #[serde(rename = "statusMappings", skip_serializing_if = "Option::is_none")]
    pub status_mappings: Option<Vec<crate::models::StatusMapping>>,
}

impl PublishDraftWorkflowScheme {
    /// Details about the status mappings for publishing a draft workflow scheme.
    pub fn new() -> PublishDraftWorkflowScheme {
        PublishDraftWorkflowScheme {
            status_mappings: None,
        }
    }
}


