/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ConfigurationTimeTrackingConfiguration : The configuration of time tracking.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationTimeTrackingConfiguration {
    /// The default unit of time applied to logged time.
    #[serde(rename = "defaultUnit")]
    pub default_unit: DefaultUnit,
    /// The format that will appear on an issue's *Time Spent* field.
    #[serde(rename = "timeFormat")]
    pub time_format: TimeFormat,
    /// The number of days in a working week.
    #[serde(rename = "workingDaysPerWeek")]
    pub working_days_per_week: f64,
    /// The number of hours in a working day.
    #[serde(rename = "workingHoursPerDay")]
    pub working_hours_per_day: f64,
}

impl ConfigurationTimeTrackingConfiguration {
    /// The configuration of time tracking.
    pub fn new(default_unit: DefaultUnit, time_format: TimeFormat, working_days_per_week: f64, working_hours_per_day: f64) -> ConfigurationTimeTrackingConfiguration {
        ConfigurationTimeTrackingConfiguration {
            default_unit,
            time_format,
            working_days_per_week,
            working_hours_per_day,
        }
    }
}

/// The default unit of time applied to logged time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultUnit {
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
}

impl Default for DefaultUnit {
    fn default() -> DefaultUnit {
        Self::Minute
    }
}
/// The format that will appear on an issue's *Time Spent* field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeFormat {
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "hours")]
    Hours,
}

impl Default for TimeFormat {
    fn default() -> TimeFormat {
        Self::Pretty
    }
}

