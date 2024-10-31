# \ModelsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_new_model_models_add_post**](ModelsApi.md#add_new_model_models_add_post) | **POST** /models/add | Add New Model
[**delete_model_by_id_models_delete_delete**](ModelsApi.md#delete_model_by_id_models_delete_delete) | **DELETE** /models/delete | Delete Model By Id
[**get_models_models_get**](ModelsApi.md#get_models_models_get) | **GET** /models/ | Get Models
[**update_model_by_id_models_update_post**](ModelsApi.md#update_model_by_id_models_update_post) | **POST** /models/update | Update Model By Id



## add_new_model_models_add_post

> models::ModelModel add_new_model_models_add_post(model_form)
Add New Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_form** | [**ModelForm**](ModelForm.md) |  | [required] |

### Return type

[**models::ModelModel**](ModelModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_model_by_id_models_delete_delete

> bool delete_model_by_id_models_delete_delete(id)
Delete Model By Id

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


## get_models_models_get

> Vec<models::ModelResponse> get_models_models_get(id)
Get Models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |

### Return type

[**Vec<models::ModelResponse>**](ModelResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_model_by_id_models_update_post

> models::ModelModel update_model_by_id_models_update_post(id, model_form)
Update Model By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**model_form** | [**ModelForm**](ModelForm.md) |  | [required] |

### Return type

[**models::ModelModel**](ModelModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

