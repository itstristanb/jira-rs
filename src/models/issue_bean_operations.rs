/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueBeanOperations : The operations that can be performed on the issue.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueBeanOperations {
    /// Details of the link groups defining issue operations.
    #[serde(rename = "linkGroups", skip_serializing_if = "Option::is_none")]
    pub link_groups: Option<Vec<crate::models::LinkGroup>>,
}

impl IssueBeanOperations {
    /// The operations that can be performed on the issue.
    pub fn new() -> IssueBeanOperations {
        IssueBeanOperations {
            link_groups: None,
        }
    }
}


