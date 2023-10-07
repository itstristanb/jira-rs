/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueSecuritySchemeToProjectMapping : Details about an project using security scheme mapping.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueSecuritySchemeToProjectMapping {
    #[serde(rename = "issueSecuritySchemeId", skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme_id: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl IssueSecuritySchemeToProjectMapping {
    /// Details about an project using security scheme mapping.
    pub fn new() -> IssueSecuritySchemeToProjectMapping {
        IssueSecuritySchemeToProjectMapping {
            issue_security_scheme_id: None,
            project_id: None,
        }
    }
}


