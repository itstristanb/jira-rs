/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeWorkflowMapping : Details about the mapping between an issue type and a workflow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeWorkflowMapping {
    /// The ID of the issue type. Not required if updating the issue type-workflow mapping.
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    /// Set to true to create or update the draft of a workflow scheme and update the mapping in the draft, when the workflow scheme cannot be edited. Defaults to `false`. Only applicable when updating the workflow-issue types mapping.
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    /// The name of the workflow.
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}

impl IssueTypeWorkflowMapping {
    /// Details about the mapping between an issue type and a workflow.
    pub fn new() -> IssueTypeWorkflowMapping {
        IssueTypeWorkflowMapping {
            issue_type: None,
            update_draft_if_needed: None,
            workflow: None,
        }
    }
}

