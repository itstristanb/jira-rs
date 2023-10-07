/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorklogIdsRequestBean {
    /// A list of worklog IDs.
    #[serde(rename = "ids")]
    pub ids: Vec<i64>,
}

impl WorklogIdsRequestBean {
    pub fn new(ids: Vec<i64>) -> WorklogIdsRequestBean {
        WorklogIdsRequestBean {
            ids,
        }
    }
}


