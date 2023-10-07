/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldCreatedContextOptionsList : A list of custom field options for a context.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldCreatedContextOptionsList {
    /// The created custom field options.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::CustomFieldContextOption>>,
}

impl CustomFieldCreatedContextOptionsList {
    /// A list of custom field options for a context.
    pub fn new() -> CustomFieldCreatedContextOptionsList {
        CustomFieldCreatedContextOptionsList {
            options: None,
        }
    }
}


