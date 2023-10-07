/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WebhookRegistrationDetails : Details of webhooks to register.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookRegistrationDetails {
    /// The URL that specifies where to send the webhooks. This URL must use the same base URL as the Connect app. Only a single URL per app is allowed to be registered.
    #[serde(rename = "url")]
    pub url: String,
    /// A list of webhooks.
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<crate::models::WebhookDetails>,
}

impl WebhookRegistrationDetails {
    /// Details of webhooks to register.
    pub fn new(url: String, webhooks: Vec<crate::models::WebhookDetails>) -> WebhookRegistrationDetails {
        WebhookRegistrationDetails {
            url,
            webhooks,
        }
    }
}

