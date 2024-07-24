# \ManageIndexesApi

All URIs are relative to *https://api.pinecone.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**configure_index**](ManageIndexesApi.md#configure_index) | **PATCH** /indexes/{index_name} | Configure an index
[**create_collection**](ManageIndexesApi.md#create_collection) | **POST** /collections | Create a collection
[**create_index**](ManageIndexesApi.md#create_index) | **POST** /indexes | Create an index
[**delete_collection**](ManageIndexesApi.md#delete_collection) | **DELETE** /collections/{collection_name} | Delete a collection
[**delete_index**](ManageIndexesApi.md#delete_index) | **DELETE** /indexes/{index_name} | Delete an index
[**describe_collection**](ManageIndexesApi.md#describe_collection) | **GET** /collections/{collection_name} | Describe a collection
[**describe_index**](ManageIndexesApi.md#describe_index) | **GET** /indexes/{index_name} | Describe an index
[**list_collections**](ManageIndexesApi.md#list_collections) | **GET** /collections | List collections
[**list_indexes**](ManageIndexesApi.md#list_indexes) | **GET** /indexes | List indexes



## configure_index

> models::IndexModel configure_index(index_name, configure_index_request)
Configure an index

This operation configures an existing index.   For serverless indexes, you can configure only index deletion protection. For pod-based indexes, you can configure the pod size, number of replicas, and index deletion protection.   It is not possible to change the pod type of a pod-based index. However, you can create a collection from a pod-based index and then [create a new pod-based index with a different pod type](http://docs.pinecone.io/guides/indexes/create-an-index#create-an-index-from-a-collection) from the collection. For guidance and examples, see [Configure an index](http://docs.pinecone.io/guides/indexes/configure-an-index).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** | The name of the index to configure. | [required] |
**configure_index_request** | [**ConfigureIndexRequest**](ConfigureIndexRequest.md) | The desired pod size and replica configuration for the index. | [required] |

### Return type

[**models::IndexModel**](IndexModel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_collection

> models::CollectionModel create_collection(create_collection_request)
Create a collection

This operation creates a Pinecone collection.    Serverless indexes do not support collections. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_collection_request** | [**CreateCollectionRequest**](CreateCollectionRequest.md) | The desired configuration for the collection. | [required] |

### Return type

[**models::CollectionModel**](CollectionModel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_index

> models::IndexModel create_index(create_index_request)
Create an index

This operation deploys a Pinecone index. This is where you specify the measure of similarity, the dimension of vectors to be stored in the index, which cloud provider you would like to deploy with, and more.    For guidance and examples, see [Create an index](https://docs.pinecone.io/guides/indexes/create-an-index#create-a-serverless-index). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_index_request** | [**CreateIndexRequest**](CreateIndexRequest.md) | The desired configuration for the index. | [required] |

### Return type

[**models::IndexModel**](IndexModel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> delete_collection(collection_name)
Delete a collection

This operation deletes an existing collection. Serverless indexes do not support collections. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_index

> delete_index(index_name)
Delete an index

This operation deletes an existing index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** | The name of the index to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## describe_collection

> models::CollectionModel describe_collection(collection_name)
Describe a collection

This operation gets a description of a collection. Serverless indexes do not support collections. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to be described. | [required] |

### Return type

[**models::CollectionModel**](CollectionModel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## describe_index

> models::IndexModel describe_index(index_name)
Describe an index

Get a description of an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** | The name of the index to be described. | [required] |

### Return type

[**models::IndexModel**](IndexModel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_collections

> models::CollectionList list_collections()
List collections

This operation returns a list of all collections in a project. Serverless indexes do not support collections. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CollectionList**](CollectionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexes

> models::IndexList list_indexes()
List indexes

This operation returns a list of all indexes in a project.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IndexList**](IndexList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

