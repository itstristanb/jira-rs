/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectIdentifiers : Identifiers for a project.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIdentifiers {
    /// The ID of the created project.
    #[serde(rename = "id")]
    pub id: i64,
    /// The key of the created project.
    #[serde(rename = "key")]
    pub key: String,
    /// The URL of the created project.
    #[serde(rename = "self")]
    pub param_self: String,
}

impl ProjectIdentifiers {
    /// Identifiers for a project.
    pub fn new(id: i64, key: String, param_self: String) -> ProjectIdentifiers {
        ProjectIdentifiers {
            id,
            key,
            param_self,
        }
    }
}


