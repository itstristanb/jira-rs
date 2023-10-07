/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowSchemeUpdateRequiredMappingsRequest : The request payload to get the required mappings for updating a workflow scheme.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeUpdateRequiredMappingsRequest {
    /// The ID of the new default workflow for this workflow scheme. Only used in global-scoped workflow schemes. If it isn't specified, is set to *Jira Workflow (jira)*.
    #[serde(rename = "defaultWorkflowId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_workflow_id: Option<Option<String>>,
    /// The ID of the workflow scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The new workflow to issue type mappings for this workflow scheme.
    #[serde(rename = "workflowsForIssueTypes")]
    pub workflows_for_issue_types: Vec<crate::models::WorkflowSchemeAssociation>,
}

impl WorkflowSchemeUpdateRequiredMappingsRequest {
    /// The request payload to get the required mappings for updating a workflow scheme.
    pub fn new(id: String, workflows_for_issue_types: Vec<crate::models::WorkflowSchemeAssociation>) -> WorkflowSchemeUpdateRequiredMappingsRequest {
        WorkflowSchemeUpdateRequiredMappingsRequest {
            default_workflow_id: None,
            id,
            workflows_for_issue_types,
        }
    }
}

