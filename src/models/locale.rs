/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Locale : Details of a locale.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Locale {
    /// The locale code. The Java the locale format is used: a two character language code (ISO 639), an underscore, and two letter country code (ISO 3166). For example, en\\_US represents a locale of English (United States). Required on create.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl Locale {
    /// Details of a locale.
    pub fn new() -> Locale {
        Locale {
            locale: None,
        }
    }
}


