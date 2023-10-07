/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldConfigurationIssueTypeItem : The field configuration for an issue type.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationIssueTypeItem {
    /// The ID of the field configuration.
    #[serde(rename = "fieldConfigurationId")]
    pub field_configuration_id: String,
    /// The ID of the field configuration scheme.
    #[serde(rename = "fieldConfigurationSchemeId")]
    pub field_configuration_scheme_id: String,
    /// The ID of the issue type or *default*. When set to *default* this field configuration issue type item applies to all issue types without a field configuration.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
}

impl FieldConfigurationIssueTypeItem {
    /// The field configuration for an issue type.
    pub fn new(field_configuration_id: String, field_configuration_scheme_id: String, issue_type_id: String) -> FieldConfigurationIssueTypeItem {
        FieldConfigurationIssueTypeItem {
            field_configuration_id,
            field_configuration_scheme_id,
            issue_type_id,
        }
    }
}


