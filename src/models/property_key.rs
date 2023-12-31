/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PropertyKey : Property key details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyKey {
    /// The key of the property.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The URL of the property.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl PropertyKey {
    /// Property key details.
    pub fn new() -> PropertyKey {
        PropertyKey {
            key: None,
            param_self: None,
        }
    }
}


