/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryClause : A JQL query clause.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryClause {
    /// The list of nested clauses.
    #[serde(rename = "clauses")]
    pub clauses: Vec<crate::models::JqlQueryClause>,
    /// The operator applied to the field.
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "field")]
    pub field: Box<crate::models::JqlQueryField>,
    #[serde(rename = "operand")]
    pub operand: Box<crate::models::JqlQueryClauseOperand>,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<crate::models::JqlQueryClauseTimePredicate>,
}

impl JqlQueryClause {
    /// A JQL query clause.
    pub fn new(clauses: Vec<crate::models::JqlQueryClause>, operator: Operator, field: crate::models::JqlQueryField, operand: crate::models::JqlQueryClauseOperand, predicates: Vec<crate::models::JqlQueryClauseTimePredicate>) -> JqlQueryClause {
        JqlQueryClause {
            clauses,
            operator,
            field: Box::new(field),
            operand: Box::new(operand),
            predicates,
        }
    }
}

/// The operator applied to the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "changed")]
    Changed,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Changed
    }
}

