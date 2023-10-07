/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectProjectCategory : The category the project belongs to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectCategory {
    /// The description of the project category.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the project category.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the project category. Required on create, optional on update.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the project category.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl ProjectProjectCategory {
    /// The category the project belongs to.
    pub fn new() -> ProjectProjectCategory {
        ProjectProjectCategory {
            description: None,
            id: None,
            name: None,
            param_self: None,
        }
    }
}


