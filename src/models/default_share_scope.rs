/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DefaultShareScope : Details of the scope of the default sharing for new filters and dashboards.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultShareScope {
    /// The scope of the default sharing for new filters and dashboards:   *  `AUTHENTICATED` Shared with all logged-in users.  *  `GLOBAL` Shared with all logged-in users. This shows as `AUTHENTICATED` in the response.  *  `PRIVATE` Not shared with any users.
    #[serde(rename = "scope")]
    pub scope: Scope,
}

impl DefaultShareScope {
    /// Details of the scope of the default sharing for new filters and dashboards.
    pub fn new(scope: Scope) -> DefaultShareScope {
        DefaultShareScope {
            scope,
        }
    }
}

/// The scope of the default sharing for new filters and dashboards:   *  `AUTHENTICATED` Shared with all logged-in users.  *  `GLOBAL` Shared with all logged-in users. This shows as `AUTHENTICATED` in the response.  *  `PRIVATE` Not shared with any users.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "AUTHENTICATED")]
    Authenticated,
    #[serde(rename = "PRIVATE")]
    Private,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Global
    }
}

