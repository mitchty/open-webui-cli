# \FunctionsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_function_functions_create_post**](FunctionsApi.md#create_new_function_functions_create_post) | **POST** /functions/create | Create New Function
[**delete_function_by_id_functions_id_id_delete_delete**](FunctionsApi.md#delete_function_by_id_functions_id_id_delete_delete) | **DELETE** /functions/id/{id}/delete | Delete Function By Id
[**get_function_by_id_functions_id_id_get**](FunctionsApi.md#get_function_by_id_functions_id_id_get) | **GET** /functions/id/{id} | Get Function By Id
[**get_function_user_valves_by_id_functions_id_id_valves_user_get**](FunctionsApi.md#get_function_user_valves_by_id_functions_id_id_valves_user_get) | **GET** /functions/id/{id}/valves/user | Get Function User Valves By Id
[**get_function_user_valves_spec_by_id_functions_id_id_valves_user_spec_get**](FunctionsApi.md#get_function_user_valves_spec_by_id_functions_id_id_valves_user_spec_get) | **GET** /functions/id/{id}/valves/user/spec | Get Function User Valves Spec By Id
[**get_function_valves_by_id_functions_id_id_valves_get**](FunctionsApi.md#get_function_valves_by_id_functions_id_id_valves_get) | **GET** /functions/id/{id}/valves | Get Function Valves By Id
[**get_function_valves_spec_by_id_functions_id_id_valves_spec_get**](FunctionsApi.md#get_function_valves_spec_by_id_functions_id_id_valves_spec_get) | **GET** /functions/id/{id}/valves/spec | Get Function Valves Spec By Id
[**get_functions_functions_export_get**](FunctionsApi.md#get_functions_functions_export_get) | **GET** /functions/export | Get Functions
[**get_functions_functions_get**](FunctionsApi.md#get_functions_functions_get) | **GET** /functions/ | Get Functions
[**toggle_function_by_id_functions_id_id_toggle_post**](FunctionsApi.md#toggle_function_by_id_functions_id_id_toggle_post) | **POST** /functions/id/{id}/toggle | Toggle Function By Id
[**toggle_global_by_id_functions_id_id_toggle_global_post**](FunctionsApi.md#toggle_global_by_id_functions_id_id_toggle_global_post) | **POST** /functions/id/{id}/toggle/global | Toggle Global By Id
[**update_function_by_id_functions_id_id_update_post**](FunctionsApi.md#update_function_by_id_functions_id_id_update_post) | **POST** /functions/id/{id}/update | Update Function By Id
[**update_function_user_valves_by_id_functions_id_id_valves_user_update_post**](FunctionsApi.md#update_function_user_valves_by_id_functions_id_id_valves_user_update_post) | **POST** /functions/id/{id}/valves/user/update | Update Function User Valves By Id
[**update_function_valves_by_id_functions_id_id_valves_update_post**](FunctionsApi.md#update_function_valves_by_id_functions_id_id_valves_update_post) | **POST** /functions/id/{id}/valves/update | Update Function Valves By Id



## create_new_function_functions_create_post

> models::FunctionResponse create_new_function_functions_create_post(function_form)
Create New Function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_form** | [**FunctionForm**](FunctionForm.md) |  | [required] |

### Return type

[**models::FunctionResponse**](FunctionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_function_by_id_functions_id_id_delete_delete

> bool delete_function_by_id_functions_id_id_delete_delete(id)
Delete Function By Id

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


## get_function_by_id_functions_id_id_get

> models::FunctionModel get_function_by_id_functions_id_id_get(id)
Get Function By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FunctionModel**](FunctionModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_function_user_valves_by_id_functions_id_id_valves_user_get

> serde_json::Value get_function_user_valves_by_id_functions_id_id_valves_user_get(id)
Get Function User Valves By Id

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


## get_function_user_valves_spec_by_id_functions_id_id_valves_user_spec_get

> serde_json::Value get_function_user_valves_spec_by_id_functions_id_id_valves_user_spec_get(id)
Get Function User Valves Spec By Id

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


## get_function_valves_by_id_functions_id_id_valves_get

> serde_json::Value get_function_valves_by_id_functions_id_id_valves_get(id)
Get Function Valves By Id

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


## get_function_valves_spec_by_id_functions_id_id_valves_spec_get

> serde_json::Value get_function_valves_spec_by_id_functions_id_id_valves_spec_get(id)
Get Function Valves Spec By Id

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


## get_functions_functions_export_get

> Vec<models::FunctionModel> get_functions_functions_export_get()
Get Functions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FunctionModel>**](FunctionModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_functions_functions_get

> Vec<models::FunctionResponse> get_functions_functions_get()
Get Functions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FunctionResponse>**](FunctionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_function_by_id_functions_id_id_toggle_post

> models::FunctionModel toggle_function_by_id_functions_id_id_toggle_post(id)
Toggle Function By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FunctionModel**](FunctionModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_global_by_id_functions_id_id_toggle_global_post

> models::FunctionModel toggle_global_by_id_functions_id_id_toggle_global_post(id)
Toggle Global By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FunctionModel**](FunctionModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_function_by_id_functions_id_id_update_post

> models::FunctionModel update_function_by_id_functions_id_id_update_post(id, function_form)
Update Function By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**function_form** | [**FunctionForm**](FunctionForm.md) |  | [required] |

### Return type

[**models::FunctionModel**](FunctionModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_function_user_valves_by_id_functions_id_id_valves_user_update_post

> serde_json::Value update_function_user_valves_by_id_functions_id_id_valves_user_update_post(id, body)
Update Function User Valves By Id

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


## update_function_valves_by_id_functions_id_id_valves_update_post

> serde_json::Value update_function_valves_by_id_functions_id_id_valves_update_post(id, body)
Update Function Valves By Id

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

