/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionsAnalysis : Details about the analysed Jira expression.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraExpressionsAnalysis {
    /// The results of Jira expressions analysis.
    #[serde(rename = "results")]
    pub results: Vec<crate::models::JiraExpressionAnalysis>,
}

impl JiraExpressionsAnalysis {
    /// Details about the analysed Jira expression.
    pub fn new(results: Vec<crate::models::JiraExpressionAnalysis>) -> JiraExpressionsAnalysis {
        JiraExpressionsAnalysis {
            results,
        }
    }
}


