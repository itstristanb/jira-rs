/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectDetails : Details about a project.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDetails {
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<Box<crate::models::ProjectAvatarUrls>>,
    /// The ID of the project.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the project.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectCategory", skip_serializing_if = "Option::is_none")]
    pub project_category: Option<Box<crate::models::ProjectDetailsProjectCategory>>,
    /// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project.
    #[serde(rename = "projectTypeKey", skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<ProjectTypeKey>,
    /// The URL of the project details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Whether or not the project is simplified.
    #[serde(rename = "simplified", skip_serializing_if = "Option::is_none")]
    pub simplified: Option<bool>,
}

impl ProjectDetails {
    /// Details about a project.
    pub fn new() -> ProjectDetails {
        ProjectDetails {
            avatar_urls: None,
            id: None,
            key: None,
            name: None,
            project_category: None,
            project_type_key: None,
            param_self: None,
            simplified: None,
        }
    }
}

/// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectTypeKey {
    #[serde(rename = "software")]
    Software,
    #[serde(rename = "service_desk")]
    ServiceDesk,
    #[serde(rename = "business")]
    Business,
}

impl Default for ProjectTypeKey {
    fn default() -> ProjectTypeKey {
        Self::Software
    }
}

