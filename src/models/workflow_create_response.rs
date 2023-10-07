/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowCreateResponse : Details of the created workflows and statuses.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCreateResponse {
    /// List of created statuses.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<crate::models::JiraWorkflowStatus>>,
    /// List of created workflows.
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<crate::models::JiraWorkflow>>,
}

impl WorkflowCreateResponse {
    /// Details of the created workflows and statuses.
    pub fn new() -> WorkflowCreateResponse {
        WorkflowCreateResponse {
            statuses: None,
            workflows: None,
        }
    }
}


