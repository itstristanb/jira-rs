/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// VersionIssuesStatusForFixVersion : If the expand option `issuesstatus` is used, returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionIssuesStatusForFixVersion {
    /// Count of issues with status *done*.
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<i64>,
    /// Count of issues with status *in progress*.
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    /// Count of issues with status *to do*.
    #[serde(rename = "toDo", skip_serializing_if = "Option::is_none")]
    pub to_do: Option<i64>,
    /// Count of issues with a status other than *to do*, *in progress*, and *done*.
    #[serde(rename = "unmapped", skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<i64>,
}

impl VersionIssuesStatusForFixVersion {
    /// If the expand option `issuesstatus` is used, returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.
    pub fn new() -> VersionIssuesStatusForFixVersion {
        VersionIssuesStatusForFixVersion {
            done: None,
            in_progress: None,
            to_do: None,
            unmapped: None,
        }
    }
}


