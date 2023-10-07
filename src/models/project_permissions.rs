/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectPermissions : User permissions on the project



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectPermissions {
    /// Whether the logged user can edit the project.
    #[serde(rename = "canEdit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
}

impl ProjectPermissions {
    /// User permissions on the project
    pub fn new() -> ProjectPermissions {
        ProjectPermissions {
            can_edit: None,
        }
    }
}


