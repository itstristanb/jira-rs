/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssuePickerSuggestions : A list of issues suggested for use in auto-completion.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuePickerSuggestions {
    /// A list of issues for an issue type suggested for use in auto-completion.
    #[serde(rename = "sections", skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<crate::models::IssuePickerSuggestionsIssueType>>,
}

impl IssuePickerSuggestions {
    /// A list of issues suggested for use in auto-completion.
    pub fn new() -> IssuePickerSuggestions {
        IssuePickerSuggestions {
            sections: None,
        }
    }
}


