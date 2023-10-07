/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardGadgetUpdateRequest : The details of the gadget to update.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardGadgetUpdateRequest {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::DashboardGadgetPosition>>,
    /// The title of the gadget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl DashboardGadgetUpdateRequest {
    /// The details of the gadget to update.
    pub fn new() -> DashboardGadgetUpdateRequest {
        DashboardGadgetUpdateRequest {
            color: None,
            position: None,
            title: None,
        }
    }
}

