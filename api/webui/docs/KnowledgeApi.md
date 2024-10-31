# \KnowledgeApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_file_to_knowledge_by_id_knowledge_id_file_add_post**](KnowledgeApi.md#add_file_to_knowledge_by_id_knowledge_id_file_add_post) | **POST** /knowledge/{id}/file/add | Add File To Knowledge By Id
[**create_new_knowledge_knowledge_create_post**](KnowledgeApi.md#create_new_knowledge_knowledge_create_post) | **POST** /knowledge/create | Create New Knowledge
[**delete_knowledge_by_id_knowledge_id_delete_delete**](KnowledgeApi.md#delete_knowledge_by_id_knowledge_id_delete_delete) | **DELETE** /knowledge/{id}/delete | Delete Knowledge By Id
[**get_knowledge_by_id_knowledge_id_get**](KnowledgeApi.md#get_knowledge_by_id_knowledge_id_get) | **GET** /knowledge/{id} | Get Knowledge By Id
[**get_knowledge_items_knowledge_get**](KnowledgeApi.md#get_knowledge_items_knowledge_get) | **GET** /knowledge/ | Get Knowledge Items
[**remove_file_from_knowledge_by_id_knowledge_id_file_remove_post**](KnowledgeApi.md#remove_file_from_knowledge_by_id_knowledge_id_file_remove_post) | **POST** /knowledge/{id}/file/remove | Remove File From Knowledge By Id
[**reset_knowledge_by_id_knowledge_id_reset_post**](KnowledgeApi.md#reset_knowledge_by_id_knowledge_id_reset_post) | **POST** /knowledge/{id}/reset | Reset Knowledge By Id
[**update_file_from_knowledge_by_id_knowledge_id_file_update_post**](KnowledgeApi.md#update_file_from_knowledge_by_id_knowledge_id_file_update_post) | **POST** /knowledge/{id}/file/update | Update File From Knowledge By Id
[**update_knowledge_by_id_knowledge_id_update_post**](KnowledgeApi.md#update_knowledge_by_id_knowledge_id_update_post) | **POST** /knowledge/{id}/update | Update Knowledge By Id



## add_file_to_knowledge_by_id_knowledge_id_file_add_post

> models::KnowledgeFilesResponse add_file_to_knowledge_by_id_knowledge_id_file_add_post(id, knowledge_file_id_form)
Add File To Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**knowledge_file_id_form** | [**KnowledgeFileIdForm**](KnowledgeFileIdForm.md) |  | [required] |

### Return type

[**models::KnowledgeFilesResponse**](KnowledgeFilesResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_new_knowledge_knowledge_create_post

> models::KnowledgeResponse create_new_knowledge_knowledge_create_post(knowledge_form)
Create New Knowledge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_form** | [**KnowledgeForm**](KnowledgeForm.md) |  | [required] |

### Return type

[**models::KnowledgeResponse**](KnowledgeResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_knowledge_by_id_knowledge_id_delete_delete

> bool delete_knowledge_by_id_knowledge_id_delete_delete(id)
Delete Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_by_id_knowledge_id_get

> models::KnowledgeFilesResponse get_knowledge_by_id_knowledge_id_get(id)
Get Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::KnowledgeFilesResponse**](KnowledgeFilesResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_items_knowledge_get

> models::ResponseGetKnowledgeItemsKnowledgeGet get_knowledge_items_knowledge_get(id)
Get Knowledge Items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |

### Return type

[**models::ResponseGetKnowledgeItemsKnowledgeGet**](Response_Get_Knowledge_Items_Knowledge__Get.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_file_from_knowledge_by_id_knowledge_id_file_remove_post

> models::KnowledgeFilesResponse remove_file_from_knowledge_by_id_knowledge_id_file_remove_post(id, knowledge_file_id_form)
Remove File From Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**knowledge_file_id_form** | [**KnowledgeFileIdForm**](KnowledgeFileIdForm.md) |  | [required] |

### Return type

[**models::KnowledgeFilesResponse**](KnowledgeFilesResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_knowledge_by_id_knowledge_id_reset_post

> models::KnowledgeResponse reset_knowledge_by_id_knowledge_id_reset_post(id)
Reset Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::KnowledgeResponse**](KnowledgeResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_file_from_knowledge_by_id_knowledge_id_file_update_post

> models::KnowledgeFilesResponse update_file_from_knowledge_by_id_knowledge_id_file_update_post(id, knowledge_file_id_form)
Update File From Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**knowledge_file_id_form** | [**KnowledgeFileIdForm**](KnowledgeFileIdForm.md) |  | [required] |

### Return type

[**models::KnowledgeFilesResponse**](KnowledgeFilesResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_knowledge_by_id_knowledge_id_update_post

> models::KnowledgeFilesResponse update_knowledge_by_id_knowledge_id_update_post(id, knowledge_update_form)
Update Knowledge By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**knowledge_update_form** | [**KnowledgeUpdateForm**](KnowledgeUpdateForm.md) |  | [required] |

### Return type

[**models::KnowledgeFilesResponse**](KnowledgeFilesResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

