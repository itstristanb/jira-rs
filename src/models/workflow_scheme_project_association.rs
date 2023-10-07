/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowSchemeProjectAssociation : An associated workflow scheme and project.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeProjectAssociation {
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The ID of the workflow scheme. If the workflow scheme ID is `null`, the operation assigns the default workflow scheme.
    #[serde(rename = "workflowSchemeId", skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<String>,
}

impl WorkflowSchemeProjectAssociation {
    /// An associated workflow scheme and project.
    pub fn new(project_id: String) -> WorkflowSchemeProjectAssociation {
        WorkflowSchemeProjectAssociation {
            project_id,
            workflow_scheme_id: None,
        }
    }
}


