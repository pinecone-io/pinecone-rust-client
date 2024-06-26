# \InferenceApi

All URIs are relative to *https://api.pinecone.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**embed**](InferenceApi.md#embed) | **POST** /embed | Embed data



## embed

> models::EmbeddingsList embed(embed_request)
Embed data

Generate embeddings for input data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**embed_request** | Option<[**EmbedRequest**](EmbedRequest.md)> | Generate embeddings for inputs |  |

### Return type

[**models::EmbeddingsList**](EmbeddingsList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

