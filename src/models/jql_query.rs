/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQuery : A parsed JQL query.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQuery {
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Box<crate::models::JqlQueryOrderByClause>>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<crate::models::JqlQueryClause>>,
}

impl JqlQuery {
    /// A parsed JQL query.
    pub fn new() -> JqlQuery {
        JqlQuery {
            order_by: None,
            r#where: None,
        }
    }
}


