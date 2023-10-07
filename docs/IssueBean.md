# IssueBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**changelog** | Option<[**crate::models::IssueBeanChangelog**](IssueBean_changelog.md)> |  | [optional]
**editmeta** | Option<[**crate::models::IssueBeanEditmeta**](IssueBean_editmeta.md)> |  | [optional]
**expand** | Option<**String**> | Expand options that include additional issue details in the response. | [optional][readonly]
**fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**fields_to_include** | Option<[**crate::models::IncludedFields**](IncludedFields.md)> |  | [optional]
**id** | Option<**String**> | The ID of the issue. | [optional][readonly]
**key** | Option<**String**> | The key of the issue. | [optional][readonly]
**names** | Option<**::std::collections::HashMap<String, String>**> | The ID and name of each field present on the issue. | [optional][readonly]
**operations** | Option<[**crate::models::IssueBeanOperations**](IssueBean_operations.md)> |  | [optional]
**properties** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Details of the issue properties identified in the request. | [optional][readonly]
**rendered_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The rendered value of each field present on the issue. | [optional][readonly]
**schema** | Option<[**::std::collections::HashMap<String, crate::models::JsonTypeBean>**](JsonTypeBean.md)> | The schema describing each field present on the issue. | [optional][readonly]
**param_self** | Option<**String**> | The URL of the issue details. | [optional][readonly]
**transitions** | Option<[**Vec<crate::models::IssueTransition>**](IssueTransition.md)> | The transitions that can be performed on the issue. | [optional][readonly]
**versioned_representations** | Option<[**::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>>**](map.md)> | The versions of each field on the issue. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


