/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// TransitionUpdateDto : The transitions of this workflow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransitionUpdateDto {
    /// The post-functions of the transition.
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::models::WorkflowRuleConfiguration>>,
    #[serde(rename = "conditions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Option<Box<crate::models::ConditionGroupUpdate>>>,
    /// The custom event ID of the transition.
    #[serde(rename = "customIssueEventId", skip_serializing_if = "Option::is_none")]
    pub custom_issue_event_id: Option<String>,
    /// The description of the transition.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The statuses the transition can start from.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<crate::models::StatusReferenceAndPort>>,
    /// The ID of the transition.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the transition.
    #[serde(rename = "name")]
    pub name: String,
    /// The properties of the transition.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<Box<crate::models::StatusReferenceAndPort>>>,
    #[serde(rename = "transitionScreen", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transition_screen: Option<Option<Box<crate::models::WorkflowRuleConfiguration>>>,
    /// The triggers of the transition.
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<crate::models::WorkflowTrigger>>,
    /// The transition type.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The validators of the transition.
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<crate::models::WorkflowRuleConfiguration>>,
}

impl TransitionUpdateDto {
    /// The transitions of this workflow.
    pub fn new(id: String, name: String, r#type: Type) -> TransitionUpdateDto {
        TransitionUpdateDto {
            actions: None,
            conditions: None,
            custom_issue_event_id: None,
            description: None,
            from: None,
            id,
            name,
            properties: None,
            to: None,
            transition_screen: None,
            triggers: None,
            r#type,
            validators: None,
        }
    }
}

/// The transition type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "DIRECTED")]
    Directed,
}

impl Default for Type {
    fn default() -> Type {
        Self::Initial
    }
}

