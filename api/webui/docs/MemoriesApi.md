# \MemoriesApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_memory_memories_add_post**](MemoriesApi.md#add_memory_memories_add_post) | **POST** /memories/add | Add Memory
[**delete_memory_by_id_memories_memory_id_delete**](MemoriesApi.md#delete_memory_by_id_memories_memory_id_delete) | **DELETE** /memories/{memory_id} | Delete Memory By Id
[**delete_memory_by_user_id_memories_delete_user_delete**](MemoriesApi.md#delete_memory_by_user_id_memories_delete_user_delete) | **DELETE** /memories/delete/user | Delete Memory By User Id
[**get_embeddings_memories_ef_get**](MemoriesApi.md#get_embeddings_memories_ef_get) | **GET** /memories/ef | Get Embeddings
[**get_memories_memories_get**](MemoriesApi.md#get_memories_memories_get) | **GET** /memories/ | Get Memories
[**query_memory_memories_query_post**](MemoriesApi.md#query_memory_memories_query_post) | **POST** /memories/query | Query Memory
[**reset_memory_from_vector_db_memories_reset_post**](MemoriesApi.md#reset_memory_from_vector_db_memories_reset_post) | **POST** /memories/reset | Reset Memory From Vector Db
[**update_memory_by_id_memories_memory_id_update_post**](MemoriesApi.md#update_memory_by_id_memories_memory_id_update_post) | **POST** /memories/{memory_id}/update | Update Memory By Id



## add_memory_memories_add_post

> models::MemoryModel add_memory_memories_add_post(add_memory_form)
Add Memory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_memory_form** | [**AddMemoryForm**](AddMemoryForm.md) |  | [required] |

### Return type

[**models::MemoryModel**](MemoryModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_memory_by_id_memories_memory_id_delete

> bool delete_memory_by_id_memories_memory_id_delete(memory_id)
Delete Memory By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**memory_id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_memory_by_user_id_memories_delete_user_delete

> bool delete_memory_by_user_id_memories_delete_user_delete()
Delete Memory By User Id

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_embeddings_memories_ef_get

> serde_json::Value get_embeddings_memories_ef_get()
Get Embeddings

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_memories_memories_get

> Vec<models::MemoryModel> get_memories_memories_get()
Get Memories

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MemoryModel>**](MemoryModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_memory_memories_query_post

> serde_json::Value query_memory_memories_query_post(query_memory_form)
Query Memory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_memory_form** | [**QueryMemoryForm**](QueryMemoryForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_memory_from_vector_db_memories_reset_post

> bool reset_memory_from_vector_db_memories_reset_post()
Reset Memory From Vector Db

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_memory_by_id_memories_memory_id_update_post

> models::MemoryModel update_memory_by_id_memories_memory_id_update_post(memory_id, memory_update_model)
Update Memory By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**memory_id** | **String** |  | [required] |
**memory_update_model** | [**MemoryUpdateModel**](MemoryUpdateModel.md) |  | [required] |

### Return type

[**models::MemoryModel**](MemoryModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

