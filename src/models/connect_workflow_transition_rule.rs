/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ConnectWorkflowTransitionRule : A workflow transition rule.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectWorkflowTransitionRule {
    #[serde(rename = "configuration")]
    pub configuration: Box<crate::models::RuleConfiguration>,
    /// The ID of the transition rule.
    #[serde(rename = "id")]
    pub id: String,
    /// The key of the rule, as defined in the Connect app descriptor.
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<Box<crate::models::WorkflowTransition>>,
}

impl ConnectWorkflowTransitionRule {
    /// A workflow transition rule.
    pub fn new(configuration: crate::models::RuleConfiguration, id: String, key: String) -> ConnectWorkflowTransitionRule {
        ConnectWorkflowTransitionRule {
            configuration: Box::new(configuration),
            id,
            key,
            transition: None,
        }
    }
}


