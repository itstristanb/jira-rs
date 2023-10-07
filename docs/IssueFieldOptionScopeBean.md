# IssueFieldOptionScopeBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**global** | Option<[**crate::models::GlobalScopeBean**](GlobalScopeBean.md)> |  | [optional]
**projects** | Option<**Vec<i64>**> | DEPRECATED | [optional]
**projects2** | Option<[**Vec<crate::models::ProjectScopeBean>**](ProjectScopeBean.md)> | Defines the projects in which the option is available and the behavior of the option within each project. Specify one object per project. The behavior of the option in a project context overrides the behavior in the global context. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


