# \ToolsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_toolkit_tools_create_post**](ToolsApi.md#create_new_toolkit_tools_create_post) | **POST** /tools/create | Create New Toolkit
[**delete_toolkit_by_id_tools_id_id_delete_delete**](ToolsApi.md#delete_toolkit_by_id_tools_id_id_delete_delete) | **DELETE** /tools/id/{id}/delete | Delete Toolkit By Id
[**get_toolkit_by_id_tools_id_id_get**](ToolsApi.md#get_toolkit_by_id_tools_id_id_get) | **GET** /tools/id/{id} | Get Toolkit By Id
[**get_toolkit_user_valves_by_id_tools_id_id_valves_user_get**](ToolsApi.md#get_toolkit_user_valves_by_id_tools_id_id_valves_user_get) | **GET** /tools/id/{id}/valves/user | Get Toolkit User Valves By Id
[**get_toolkit_user_valves_spec_by_id_tools_id_id_valves_user_spec_get**](ToolsApi.md#get_toolkit_user_valves_spec_by_id_tools_id_id_valves_user_spec_get) | **GET** /tools/id/{id}/valves/user/spec | Get Toolkit User Valves Spec By Id
[**get_toolkit_valves_by_id_tools_id_id_valves_get**](ToolsApi.md#get_toolkit_valves_by_id_tools_id_id_valves_get) | **GET** /tools/id/{id}/valves | Get Toolkit Valves By Id
[**get_toolkit_valves_spec_by_id_tools_id_id_valves_spec_get**](ToolsApi.md#get_toolkit_valves_spec_by_id_tools_id_id_valves_spec_get) | **GET** /tools/id/{id}/valves/spec | Get Toolkit Valves Spec By Id
[**get_toolkits_tools_export_get**](ToolsApi.md#get_toolkits_tools_export_get) | **GET** /tools/export | Get Toolkits
[**get_toolkits_tools_get**](ToolsApi.md#get_toolkits_tools_get) | **GET** /tools/ | Get Toolkits
[**update_toolkit_by_id_tools_id_id_update_post**](ToolsApi.md#update_toolkit_by_id_tools_id_id_update_post) | **POST** /tools/id/{id}/update | Update Toolkit By Id
[**update_toolkit_user_valves_by_id_tools_id_id_valves_user_update_post**](ToolsApi.md#update_toolkit_user_valves_by_id_tools_id_id_valves_user_update_post) | **POST** /tools/id/{id}/valves/user/update | Update Toolkit User Valves By Id
[**update_toolkit_valves_by_id_tools_id_id_valves_update_post**](ToolsApi.md#update_toolkit_valves_by_id_tools_id_id_valves_update_post) | **POST** /tools/id/{id}/valves/update | Update Toolkit Valves By Id



## create_new_toolkit_tools_create_post

> models::ToolResponse create_new_toolkit_tools_create_post(tool_form)
Create New Toolkit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tool_form** | [**ToolForm**](ToolForm.md) |  | [required] |

### Return type

[**models::ToolResponse**](ToolResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_toolkit_by_id_tools_id_id_delete_delete

> bool delete_toolkit_by_id_tools_id_id_delete_delete(id)
Delete Toolkit By Id

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


## get_toolkit_by_id_tools_id_id_get

> models::ToolModel get_toolkit_by_id_tools_id_id_get(id)
Get Toolkit By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToolModel**](ToolModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_toolkit_user_valves_by_id_tools_id_id_valves_user_get

> serde_json::Value get_toolkit_user_valves_by_id_tools_id_id_valves_user_get(id)
Get Toolkit User Valves By Id

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


## get_toolkit_user_valves_spec_by_id_tools_id_id_valves_user_spec_get

> serde_json::Value get_toolkit_user_valves_spec_by_id_tools_id_id_valves_user_spec_get(id)
Get Toolkit User Valves Spec By Id

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


## get_toolkit_valves_by_id_tools_id_id_valves_get

> serde_json::Value get_toolkit_valves_by_id_tools_id_id_valves_get(id)
Get Toolkit Valves By Id

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


## get_toolkit_valves_spec_by_id_tools_id_id_valves_spec_get

> serde_json::Value get_toolkit_valves_spec_by_id_tools_id_id_valves_spec_get(id)
Get Toolkit Valves Spec By Id

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


## get_toolkits_tools_export_get

> Vec<models::ToolModel> get_toolkits_tools_export_get()
Get Toolkits

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ToolModel>**](ToolModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_toolkits_tools_get

> Vec<models::ToolResponse> get_toolkits_tools_get()
Get Toolkits

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ToolResponse>**](ToolResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_toolkit_by_id_tools_id_id_update_post

> models::ToolModel update_toolkit_by_id_tools_id_id_update_post(id, tool_form)
Update Toolkit By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tool_form** | [**ToolForm**](ToolForm.md) |  | [required] |

### Return type

[**models::ToolModel**](ToolModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_toolkit_user_valves_by_id_tools_id_id_valves_user_update_post

> serde_json::Value update_toolkit_user_valves_by_id_tools_id_id_valves_user_update_post(id, body)
Update Toolkit User Valves By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_toolkit_valves_by_id_tools_id_id_valves_update_post

> serde_json::Value update_toolkit_valves_by_id_tools_id_id_valves_update_post(id, body)
Update Toolkit Valves By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

