/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// StatusMappingDto : The mapping of old to new status ID for a specific project and issue type.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusMappingDto {
    /// The issue type for the status mapping.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The project for the status mapping.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The list of old and new status ID mappings for the specified project and issue type.
    #[serde(rename = "statusMigrations")]
    pub status_migrations: Vec<crate::models::StatusMigration>,
}

impl StatusMappingDto {
    /// The mapping of old to new status ID for a specific project and issue type.
    pub fn new(issue_type_id: String, project_id: String, status_migrations: Vec<crate::models::StatusMigration>) -> StatusMappingDto {
        StatusMappingDto {
            issue_type_id,
            project_id,
            status_migrations,
        }
    }
}


