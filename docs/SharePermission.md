# SharePermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group** | Option<[**crate::models::GroupName**](GroupName.md)> |  | [optional]
**id** | Option<**i64**> | The unique identifier of the share permission. | [optional][readonly]
**project** | Option<[**crate::models::Project**](Project.md)> |  | [optional]
**role** | Option<[**crate::models::ProjectRole**](ProjectRole.md)> |  | [optional]
**r#type** | **String** | The type of share permission:   *  `user` Shared with a user.  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request. | 
**user** | Option<[**crate::models::UserBean**](UserBean.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


