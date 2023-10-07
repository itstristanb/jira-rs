/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RequiredMappingByWorkflows : The list of required status mappings by workflow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredMappingByWorkflows {
    /// The ID of the source workflow.
    #[serde(rename = "sourceWorkflowId", skip_serializing_if = "Option::is_none")]
    pub source_workflow_id: Option<String>,
    /// The status IDs requiring mapping.
    #[serde(rename = "statusIds", skip_serializing_if = "Option::is_none")]
    pub status_ids: Option<Vec<String>>,
    /// The ID of the target workflow.
    #[serde(rename = "targetWorkflowId", skip_serializing_if = "Option::is_none")]
    pub target_workflow_id: Option<String>,
}

impl RequiredMappingByWorkflows {
    /// The list of required status mappings by workflow.
    pub fn new() -> RequiredMappingByWorkflows {
        RequiredMappingByWorkflows {
            source_workflow_id: None,
            status_ids: None,
            target_workflow_id: None,
        }
    }
}

