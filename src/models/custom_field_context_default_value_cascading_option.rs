/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueCascadingOption : The default value for a cascading select custom field.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueCascadingOption {
    /// The ID of the default cascading option.
    #[serde(rename = "cascadingOptionId", skip_serializing_if = "Option::is_none")]
    pub cascading_option_id: Option<String>,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default option.
    #[serde(rename = "optionId")]
    pub option_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueCascadingOption {
    /// The default value for a cascading select custom field.
    pub fn new(context_id: String, option_id: String, r#type: String) -> CustomFieldContextDefaultValueCascadingOption {
        CustomFieldContextDefaultValueCascadingOption {
            cascading_option_id: None,
            context_id,
            option_id,
            r#type,
        }
    }
}


