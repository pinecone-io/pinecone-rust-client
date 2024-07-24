# IndexModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the index. Resource name must be 1-45 characters long, start and end with an alphanumeric character, and consist only of lower case alphanumeric characters or '-'.  | 
**dimension** | **i32** | The dimensions of the vectors to be inserted in the index. | 
**metric** | **String** | The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'. | [default to Cosine]
**host** | **String** | The URL address where the index is hosted. | 
**deletion_protection** | Option<[**models::DeletionProtection**](DeletionProtection.md)> |  | [optional]
**spec** | [**models::IndexModelSpec**](IndexModel_spec.md) |  | 
**status** | [**models::IndexModelStatus**](IndexModel_status.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


