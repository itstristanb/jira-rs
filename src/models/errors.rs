/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Errors {
    #[serde(rename = "issueIsSubtask", skip_serializing_if = "Option::is_none")]
    pub issue_is_subtask: Option<Box<crate::models::Error>>,
    #[serde(rename = "issuesInArchivedProjects", skip_serializing_if = "Option::is_none")]
    pub issues_in_archived_projects: Option<Box<crate::models::Error>>,
    #[serde(rename = "issuesInUnlicensedProjects", skip_serializing_if = "Option::is_none")]
    pub issues_in_unlicensed_projects: Option<Box<crate::models::Error>>,
    #[serde(rename = "issuesNotFound", skip_serializing_if = "Option::is_none")]
    pub issues_not_found: Option<Box<crate::models::Error>>,
}

impl Errors {
    pub fn new() -> Errors {
        Errors {
            issue_is_subtask: None,
            issues_in_archived_projects: None,
            issues_in_unlicensed_projects: None,
            issues_not_found: None,
        }
    }
}


