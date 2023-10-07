/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowMetadataRestModel : Workflow metadata and usage detail.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMetadataRestModel {
    /// The description of the workflow.
    #[serde(rename = "description")]
    pub description: String,
    /// The ID of the workflow.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the workflow.
    #[serde(rename = "name")]
    pub name: String,
    /// Use the optional `workflows.usages` expand to get additional information about the projects and issue types associated with the workflows in the workflow scheme.
    #[serde(rename = "usage")]
    pub usage: Vec<crate::models::SimpleUsage>,
    #[serde(rename = "version")]
    pub version: Box<crate::models::DocumentVersion>,
}

impl WorkflowMetadataRestModel {
    /// Workflow metadata and usage detail.
    pub fn new(description: String, id: String, name: String, usage: Vec<crate::models::SimpleUsage>, version: crate::models::DocumentVersion) -> WorkflowMetadataRestModel {
        WorkflowMetadataRestModel {
            description,
            id,
            name,
            usage,
            version: Box::new(version),
        }
    }
}

