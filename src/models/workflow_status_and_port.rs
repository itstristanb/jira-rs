/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowStatusAndPort : The status reference and port that a transition is connected to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatusAndPort {
    /// The port the transition is connected to this status.
    #[serde(rename = "port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<i32>>,
    /// The reference of this status.
    #[serde(rename = "statusReference", skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
}

impl WorkflowStatusAndPort {
    /// The status reference and port that a transition is connected to.
    pub fn new() -> WorkflowStatusAndPort {
        WorkflowStatusAndPort {
            port: None,
            status_reference: None,
        }
    }
}


