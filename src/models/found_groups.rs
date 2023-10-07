/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FoundGroups : The list of groups found in a search, including header text (Showing X of Y matching groups) and total of matched groups.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FoundGroups {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::FoundGroup>>,
    /// Header text indicating the number of groups in the response and the total number of groups found in the search.
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// The total number of groups found in the search.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl FoundGroups {
    /// The list of groups found in a search, including header text (Showing X of Y matching groups) and total of matched groups.
    pub fn new() -> FoundGroups {
        FoundGroups {
            groups: None,
            header: None,
            total: None,
        }
    }
}


