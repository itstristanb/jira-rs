/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateNotificationSchemeDetails : Details of a notification scheme.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateNotificationSchemeDetails {
    /// The description of the notification scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the notification scheme. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateNotificationSchemeDetails {
    /// Details of a notification scheme.
    pub fn new() -> UpdateNotificationSchemeDetails {
        UpdateNotificationSchemeDetails {
            description: None,
            name: None,
        }
    }
}

