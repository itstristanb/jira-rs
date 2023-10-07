/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldConfigurations : Details of configurations for a custom field.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldConfigurations {
    /// The list of custom field configuration details.
    #[serde(rename = "configurations")]
    pub configurations: Vec<crate::models::ContextualConfiguration>,
}

impl CustomFieldConfigurations {
    /// Details of configurations for a custom field.
    pub fn new(configurations: Vec<crate::models::ContextualConfiguration>) -> CustomFieldConfigurations {
        CustomFieldConfigurations {
            configurations,
        }
    }
}


