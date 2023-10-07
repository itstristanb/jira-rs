/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationProperty : Details of an application property.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperty {
    /// The allowed values, if applicable.
    #[serde(rename = "allowedValues", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    /// The default value of the application property.
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The description of the application property.
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "example", skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    /// The ID of the application property. The ID and key are the same.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the application property. The ID and key are the same.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the application property.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The data type of the application property.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The new value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApplicationProperty {
    /// Details of an application property.
    pub fn new() -> ApplicationProperty {
        ApplicationProperty {
            allowed_values: None,
            default_value: None,
            desc: None,
            example: None,
            id: None,
            key: None,
            name: None,
            r#type: None,
            value: None,
        }
    }
}


