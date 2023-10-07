# Filter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A description of the filter. | [optional]
**edit_permissions** | Option<[**Vec<crate::models::SharePermission>**](SharePermission.md)> | The groups and projects that can edit the filter. | [optional]
**favourite** | Option<**bool**> | Whether the filter is selected as a favorite. | [optional]
**favourited_count** | Option<**i64**> | The count of how many users have selected this filter as a favorite, including the filter owner. | [optional][readonly]
**id** | Option<**String**> | The unique identifier for the filter. | [optional][readonly]
**jql** | Option<**String**> | The JQL query for the filter. For example, *project = SSP AND issuetype = Bug*. | [optional]
**name** | **String** | The name of the filter. Must be unique. | 
**owner** | Option<[**crate::models::FilterOwner**](Filter_owner.md)> |  | [optional]
**search_url** | Option<**String**> | A URL to view the filter results in Jira, using the [Search for issues using JQL](#api-rest-api-3-filter-search-get) operation with the filter's JQL string to return the filter results. For example, *https://your-domain.atlassian.net/rest/api/3/search?jql=project+%3D+SSP+AND+issuetype+%3D+Bug*. | [optional][readonly]
**param_self** | Option<**String**> | The URL of the filter. | [optional][readonly]
**share_permissions** | Option<[**Vec<crate::models::SharePermission>**](SharePermission.md)> | The groups and projects that the filter is shared with. | [optional]
**shared_users** | Option<[**crate::models::FilterSharedUsers**](Filter_sharedUsers.md)> |  | [optional]
**subscriptions** | Option<[**crate::models::FilterSubscriptions**](Filter_subscriptions.md)> |  | [optional]
**view_url** | Option<**String**> | A URL to view the filter results in Jira, using the ID of the filter. For example, *https://your-domain.atlassian.net/issues/?filter=10100*. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


