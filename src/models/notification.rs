/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Notification : Details about a notification.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    /// The HTML body of the email notification for the issue.
    #[serde(rename = "htmlBody", skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(rename = "restrict", skip_serializing_if = "Option::is_none")]
    pub restrict: Option<Box<crate::models::NotificationRecipientsRestrictions>>,
    /// The subject of the email notification for the issue. If this is not specified, then the subject is set to the issue key and summary.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The plain text body of the email notification for the issue.
    #[serde(rename = "textBody", skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::models::NotificationRecipients>,
}

impl Notification {
    /// Details about a notification.
    pub fn new() -> Notification {
        Notification {
            html_body: None,
            restrict: None,
            subject: None,
            text_body: None,
            to: None,
        }
    }
}


