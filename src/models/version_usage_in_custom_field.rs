/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// VersionUsageInCustomField : List of custom fields using the version.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionUsageInCustomField {
    /// The ID of the custom field.
    #[serde(rename = "customFieldId", skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<i64>,
    /// The name of the custom field.
    #[serde(rename = "fieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// Count of the issues where the custom field contains the version.
    #[serde(rename = "issueCountWithVersionInCustomField", skip_serializing_if = "Option::is_none")]
    pub issue_count_with_version_in_custom_field: Option<i64>,
}

impl VersionUsageInCustomField {
    /// List of custom fields using the version.
    pub fn new() -> VersionUsageInCustomField {
        VersionUsageInCustomField {
            custom_field_id: None,
            field_name: None,
            issue_count_with_version_in_custom_field: None,
        }
    }
}


