/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateDefaultScreenScheme : The ID of a screen scheme.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDefaultScreenScheme {
    /// The ID of the screen scheme.
    #[serde(rename = "screenSchemeId")]
    pub screen_scheme_id: String,
}

impl UpdateDefaultScreenScheme {
    /// The ID of a screen scheme.
    pub fn new(screen_scheme_id: String) -> UpdateDefaultScreenScheme {
        UpdateDefaultScreenScheme {
            screen_scheme_id,
        }
    }
}

