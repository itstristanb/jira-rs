/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldConfigurationItemsDetails : Details of field configuration items.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationItemsDetails {
    /// Details of fields in a field configuration.
    #[serde(rename = "fieldConfigurationItems")]
    pub field_configuration_items: Vec<crate::models::FieldConfigurationItem>,
}

impl FieldConfigurationItemsDetails {
    /// Details of field configuration items.
    pub fn new(field_configuration_items: Vec<crate::models::FieldConfigurationItem>) -> FieldConfigurationItemsDetails {
        FieldConfigurationItemsDetails {
            field_configuration_items,
        }
    }
}

