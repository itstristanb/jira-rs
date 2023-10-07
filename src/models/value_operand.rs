/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ValueOperand : An operand that is a user-provided value.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueOperand {
    /// Encoded value, which can be used directly in a JQL query.
    #[serde(rename = "encodedValue", skip_serializing_if = "Option::is_none")]
    pub encoded_value: Option<String>,
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
}

impl ValueOperand {
    /// An operand that is a user-provided value.
    pub fn new(value: String) -> ValueOperand {
        ValueOperand {
            encoded_value: None,
            value,
        }
    }
}

