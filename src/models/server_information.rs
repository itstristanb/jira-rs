/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ServerInformation : Details about the Jira instance.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerInformation {
    /// The base URL of the Jira instance.
    #[serde(rename = "baseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// The timestamp when the Jira version was built.
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    /// The build number of the Jira version.
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    /// The type of server deployment. This is always returned as *Cloud*.
    #[serde(rename = "deploymentType", skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// Jira instance health check results. Deprecated and no longer returned.
    #[serde(rename = "healthChecks", skip_serializing_if = "Option::is_none")]
    pub health_checks: Option<Vec<crate::models::HealthCheckResult>>,
    /// The unique identifier of the Jira version.
    #[serde(rename = "scmInfo", skip_serializing_if = "Option::is_none")]
    pub scm_info: Option<String>,
    /// The time in Jira when this request was responded to.
    #[serde(rename = "serverTime", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(rename = "serverTimeZone", skip_serializing_if = "Option::is_none")]
    pub server_time_zone: Option<Box<crate::models::ServerInformationServerTimeZone>>,
    /// The name of the Jira instance.
    #[serde(rename = "serverTitle", skip_serializing_if = "Option::is_none")]
    pub server_title: Option<String>,
    /// The version of Jira.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The major, minor, and revision version numbers of the Jira version.
    #[serde(rename = "versionNumbers", skip_serializing_if = "Option::is_none")]
    pub version_numbers: Option<Vec<i32>>,
}

impl ServerInformation {
    /// Details about the Jira instance.
    pub fn new() -> ServerInformation {
        ServerInformation {
            base_url: None,
            build_date: None,
            build_number: None,
            deployment_type: None,
            health_checks: None,
            scm_info: None,
            server_time: None,
            server_time_zone: None,
            server_title: None,
            version: None,
            version_numbers: None,
        }
    }
}


