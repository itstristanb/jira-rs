/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkedIssueFields : The fields associated with the issue.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedIssueFields {
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<crate::models::FieldsAssignee>>,
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<Box<crate::models::FieldsIssueType>>,
    #[serde(rename = "issuetype", skip_serializing_if = "Option::is_none")]
    pub issuetype: Option<Box<crate::models::IssueTypeDetails>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Box<crate::models::FieldsPriority>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::FieldsStatus>>,
    /// The summary description of the linked issue.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "timetracking", skip_serializing_if = "Option::is_none")]
    pub timetracking: Option<Box<crate::models::FieldsTimetracking>>,
}

impl LinkedIssueFields {
    /// The fields associated with the issue.
    pub fn new() -> LinkedIssueFields {
        LinkedIssueFields {
            assignee: None,
            issue_type: None,
            issuetype: None,
            priority: None,
            status: None,
            summary: None,
            timetracking: None,
        }
    }
}


