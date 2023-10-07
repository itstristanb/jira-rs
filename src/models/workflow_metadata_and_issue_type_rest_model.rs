/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowMetadataAndIssueTypeRestModel : The workflow metadata and issue type IDs which use this workflow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMetadataAndIssueTypeRestModel {
    /// The list of issue type IDs for the mapping.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    #[serde(rename = "workflow")]
    pub workflow: Box<crate::models::WorkflowMetadataRestModel>,
}

impl WorkflowMetadataAndIssueTypeRestModel {
    /// The workflow metadata and issue type IDs which use this workflow.
    pub fn new(issue_type_ids: Vec<String>, workflow: crate::models::WorkflowMetadataRestModel) -> WorkflowMetadataAndIssueTypeRestModel {
        WorkflowMetadataAndIssueTypeRestModel {
            issue_type_ids,
            workflow: Box::new(workflow),
        }
    }
}


