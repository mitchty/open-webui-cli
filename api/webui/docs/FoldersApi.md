# \FoldersApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_folder_folders_post**](FoldersApi.md#create_folder_folders_post) | **POST** /folders/ | Create Folder
[**delete_folder_by_id_folders_id_delete**](FoldersApi.md#delete_folder_by_id_folders_id_delete) | **DELETE** /folders/{id} | Delete Folder By Id
[**get_folder_by_id_folders_id_get**](FoldersApi.md#get_folder_by_id_folders_id_get) | **GET** /folders/{id} | Get Folder By Id
[**get_folders_folders_get**](FoldersApi.md#get_folders_folders_get) | **GET** /folders/ | Get Folders
[**update_folder_is_expanded_by_id_folders_id_update_expanded_post**](FoldersApi.md#update_folder_is_expanded_by_id_folders_id_update_expanded_post) | **POST** /folders/{id}/update/expanded | Update Folder Is Expanded By Id
[**update_folder_name_by_id_folders_id_update_post**](FoldersApi.md#update_folder_name_by_id_folders_id_update_post) | **POST** /folders/{id}/update | Update Folder Name By Id
[**update_folder_parent_id_by_id_folders_id_update_parent_post**](FoldersApi.md#update_folder_parent_id_by_id_folders_id_update_parent_post) | **POST** /folders/{id}/update/parent | Update Folder Parent Id By Id



## create_folder_folders_post

> serde_json::Value create_folder_folders_post(folder_form)
Create Folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_form** | [**FolderForm**](FolderForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_folder_by_id_folders_id_delete

> serde_json::Value delete_folder_by_id_folders_id_delete(id)
Delete Folder By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_by_id_folders_id_get

> models::FolderModel get_folder_by_id_folders_id_get(id)
Get Folder By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FolderModel**](FolderModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_folders_get

> Vec<models::FolderModel> get_folders_folders_get()
Get Folders

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FolderModel>**](FolderModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder_is_expanded_by_id_folders_id_update_expanded_post

> serde_json::Value update_folder_is_expanded_by_id_folders_id_update_expanded_post(id, folder_is_expanded_form)
Update Folder Is Expanded By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**folder_is_expanded_form** | [**FolderIsExpandedForm**](FolderIsExpandedForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder_name_by_id_folders_id_update_post

> serde_json::Value update_folder_name_by_id_folders_id_update_post(id, folder_form)
Update Folder Name By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**folder_form** | [**FolderForm**](FolderForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder_parent_id_by_id_folders_id_update_parent_post

> serde_json::Value update_folder_parent_id_by_id_folders_id_update_parent_post(id, folder_parent_id_form)
Update Folder Parent Id By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**folder_parent_id_form** | [**FolderParentIdForm**](FolderParentIdForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

