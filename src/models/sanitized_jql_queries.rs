/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SanitizedJqlQueries : The sanitized JQL queries for the given account IDs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SanitizedJqlQueries {
    /// The list of sanitized JQL queries.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<crate::models::SanitizedJqlQuery>>,
}

impl SanitizedJqlQueries {
    /// The sanitized JQL queries for the given account IDs.
    pub fn new() -> SanitizedJqlQueries {
        SanitizedJqlQueries {
            queries: None,
        }
    }
}


