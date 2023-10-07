/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardGadgetPosition : The position of the gadget.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardGadgetPosition {
    #[serde(rename = "The column position of the gadget.")]
    pub the_column_position_of_the_gadget_period: i32,
    #[serde(rename = "The row position of the gadget.")]
    pub the_row_position_of_the_gadget_period: i32,
}

impl DashboardGadgetPosition {
    /// The position of the gadget.
    pub fn new(the_column_position_of_the_gadget_period: i32, the_row_position_of_the_gadget_period: i32) -> DashboardGadgetPosition {
        DashboardGadgetPosition {
            the_column_position_of_the_gadget_period,
            the_row_position_of_the_gadget_period,
        }
    }
}


