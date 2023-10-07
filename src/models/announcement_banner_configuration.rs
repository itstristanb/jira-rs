/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AnnouncementBannerConfiguration : Announcement banner configuration.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnouncementBannerConfiguration {
    /// Hash of the banner data. The client detects updates by comparing hash IDs.
    #[serde(rename = "hashId", skip_serializing_if = "Option::is_none")]
    pub hash_id: Option<String>,
    /// Flag indicating if the announcement banner can be dismissed by the user.
    #[serde(rename = "isDismissible", skip_serializing_if = "Option::is_none")]
    pub is_dismissible: Option<bool>,
    /// Flag indicating if the announcement banner is enabled or not.
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// The text on the announcement banner.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Visibility of the announcement banner.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

impl AnnouncementBannerConfiguration {
    /// Announcement banner configuration.
    pub fn new() -> AnnouncementBannerConfiguration {
        AnnouncementBannerConfiguration {
            hash_id: None,
            is_dismissible: None,
            is_enabled: None,
            message: None,
            visibility: None,
        }
    }
}

/// Visibility of the announcement banner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PRIVATE")]
    Private,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}
