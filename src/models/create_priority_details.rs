/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreatePriorityDetails : Details of an issue priority.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePriorityDetails {
    /// The description of the priority.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<IconUrl>,
    /// The name of the priority. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
    /// The status color of the priority in 3-digit or 6-digit hexadecimal format.
    #[serde(rename = "statusColor")]
    pub status_color: String,
}

impl CreatePriorityDetails {
    /// Details of an issue priority.
    pub fn new(name: String, status_color: String) -> CreatePriorityDetails {
        CreatePriorityDetails {
            description: None,
            icon_url: None,
            name,
            status_color,
        }
    }
}

/// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IconUrl {
    #[serde(rename = "/images/icons/priorities/blocker.png")]
    BlockerPeriodPng,
    #[serde(rename = "/images/icons/priorities/critical.png")]
    CriticalPeriodPng,
    #[serde(rename = "/images/icons/priorities/high.png")]
    HighPeriodPng,
    #[serde(rename = "/images/icons/priorities/highest.png")]
    HighestPeriodPng,
    #[serde(rename = "/images/icons/priorities/low.png")]
    LowPeriodPng,
    #[serde(rename = "/images/icons/priorities/lowest.png")]
    LowestPeriodPng,
    #[serde(rename = "/images/icons/priorities/major.png")]
    MajorPeriodPng,
    #[serde(rename = "/images/icons/priorities/medium.png")]
    MediumPeriodPng,
    #[serde(rename = "/images/icons/priorities/minor.png")]
    MinorPeriodPng,
    #[serde(rename = "/images/icons/priorities/trivial.png")]
    TrivialPeriodPng,
}

impl Default for IconUrl {
    fn default() -> IconUrl {
        Self::BlockerPeriodPng
    }
}

