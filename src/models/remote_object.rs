/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RemoteObject : The linked item.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteObject {
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::models::Icon>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::Status>,
    /// The summary details of the item.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The title of the item.
    #[serde(rename = "title")]
    pub title: String,
    /// The URL of the item.
    #[serde(rename = "url")]
    pub url: String,
}

impl RemoteObject {
    /// The linked item.
    pub fn new(title: String, url: String) -> RemoteObject {
        RemoteObject {
            icon: None,
            status: None,
            summary: None,
            title,
            url,
        }
    }
}


