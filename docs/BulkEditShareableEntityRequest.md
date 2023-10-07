# BulkEditShareableEntityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Allowed action for bulk edit shareable entity | 
**change_owner_details** | Option<[**crate::models::BulkChangeOwnerDetails**](BulkChangeOwnerDetails.md)> |  | [optional]
**entity_ids** | **Vec<i64>** | The id list of shareable entities to be changed. | 
**extend_admin_permissions** | Option<**bool**> | Whether the actions are executed by users with Administer Jira global permission. | [optional]
**permission_details** | Option<[**crate::models::PermissionDetails**](PermissionDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


