# \FilesApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_all_files_files_all_delete**](FilesApi.md#delete_all_files_files_all_delete) | **DELETE** /files/all | Delete All Files
[**delete_file_by_id_files_id_delete**](FilesApi.md#delete_file_by_id_files_id_delete) | **DELETE** /files/{id} | Delete File By Id
[**get_file_by_id_files_id_get**](FilesApi.md#get_file_by_id_files_id_get) | **GET** /files/{id} | Get File By Id
[**get_file_content_by_id_files_id_content_file_name_get**](FilesApi.md#get_file_content_by_id_files_id_content_file_name_get) | **GET** /files/{id}/content/{file_name} | Get File Content By Id
[**get_file_content_by_id_files_id_content_get**](FilesApi.md#get_file_content_by_id_files_id_content_get) | **GET** /files/{id}/content | Get File Content By Id
[**get_file_data_content_by_id_files_id_data_content_get**](FilesApi.md#get_file_data_content_by_id_files_id_data_content_get) | **GET** /files/{id}/data/content | Get File Data Content By Id
[**list_files_files_get**](FilesApi.md#list_files_files_get) | **GET** /files/ | List Files
[**update_file_data_content_by_id_files_id_data_content_update_post**](FilesApi.md#update_file_data_content_by_id_files_id_data_content_update_post) | **POST** /files/{id}/data/content/update | Update File Data Content By Id
[**upload_file_files_post**](FilesApi.md#upload_file_files_post) | **POST** /files/ | Upload File



## delete_all_files_files_all_delete

> serde_json::Value delete_all_files_files_all_delete()
Delete All Files

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file_by_id_files_id_delete

> serde_json::Value delete_file_by_id_files_id_delete(id)
Delete File By Id

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


## get_file_by_id_files_id_get

> models::FileModel get_file_by_id_files_id_get(id)
Get File By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FileModel**](FileModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_content_by_id_files_id_content_file_name_get

> models::FileModel get_file_content_by_id_files_id_content_file_name_get(id)
Get File Content By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FileModel**](FileModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_content_by_id_files_id_content_get

> models::FileModel get_file_content_by_id_files_id_content_get(id)
Get File Content By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FileModel**](FileModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_data_content_by_id_files_id_data_content_get

> serde_json::Value get_file_data_content_by_id_files_id_data_content_get(id)
Get File Data Content By Id

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


## list_files_files_get

> Vec<models::FileModel> list_files_files_get()
List Files

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FileModel>**](FileModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_file_data_content_by_id_files_id_data_content_update_post

> serde_json::Value update_file_data_content_by_id_files_id_data_content_update_post(id, content_form)
Update File Data Content By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**content_form** | [**ContentForm**](ContentForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file_files_post

> serde_json::Value upload_file_files_post(file)
Upload File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

