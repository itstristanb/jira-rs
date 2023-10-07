/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeScreenSchemeProjectAssociation : Associated issue type screen scheme and project.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeProjectAssociation {
    /// The ID of the issue type screen scheme.
    #[serde(rename = "issueTypeScreenSchemeId", skip_serializing_if = "Option::is_none")]
    pub issue_type_screen_scheme_id: Option<String>,
    /// The ID of the project.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl IssueTypeScreenSchemeProjectAssociation {
    /// Associated issue type screen scheme and project.
    pub fn new() -> IssueTypeScreenSchemeProjectAssociation {
        IssueTypeScreenSchemeProjectAssociation {
            issue_type_screen_scheme_id: None,
            project_id: None,
        }
    }
}

