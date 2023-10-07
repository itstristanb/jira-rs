/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowReferenceStatus : The statuses referenced in the workflow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowReferenceStatus {
    /// Indicates if the status is deprecated.
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "layout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub layout: Option<Option<Box<crate::models::WorkflowStatusLayout>>>,
    /// The properties associated with the status.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
    /// The reference of the status.
    #[serde(rename = "statusReference", skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
}

impl WorkflowReferenceStatus {
    /// The statuses referenced in the workflow.
    pub fn new() -> WorkflowReferenceStatus {
        WorkflowReferenceStatus {
            deprecated: None,
            layout: None,
            properties: None,
            status_reference: None,
        }
    }
}


