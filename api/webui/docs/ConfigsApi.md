# \ConfigsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_config_configs_export_get**](ConfigsApi.md#export_config_configs_export_get) | **GET** /configs/export | Export Config
[**get_banners_configs_banners_get**](ConfigsApi.md#get_banners_configs_banners_get) | **GET** /configs/banners | Get Banners
[**import_config_configs_import_post**](ConfigsApi.md#import_config_configs_import_post) | **POST** /configs/import | Import Config
[**set_banners_configs_banners_post**](ConfigsApi.md#set_banners_configs_banners_post) | **POST** /configs/banners | Set Banners
[**set_global_default_models_configs_default_models_post**](ConfigsApi.md#set_global_default_models_configs_default_models_post) | **POST** /configs/default/models | Set Global Default Models
[**set_global_default_suggestions_configs_default_suggestions_post**](ConfigsApi.md#set_global_default_suggestions_configs_default_suggestions_post) | **POST** /configs/default/suggestions | Set Global Default Suggestions



## export_config_configs_export_get

> serde_json::Value export_config_configs_export_get()
Export Config

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


## get_banners_configs_banners_get

> Vec<models::BannerModel> get_banners_configs_banners_get()
Get Banners

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BannerModel>**](BannerModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_config_configs_import_post

> serde_json::Value import_config_configs_import_post(import_config_form)
Import Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_config_form** | [**ImportConfigForm**](ImportConfigForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_banners_configs_banners_post

> Vec<models::BannerModel> set_banners_configs_banners_post(set_banners_form)
Set Banners

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_banners_form** | [**SetBannersForm**](SetBannersForm.md) |  | [required] |

### Return type

[**Vec<models::BannerModel>**](BannerModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_global_default_models_configs_default_models_post

> String set_global_default_models_configs_default_models_post(set_default_models_form)
Set Global Default Models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_default_models_form** | [**SetDefaultModelsForm**](SetDefaultModelsForm.md) |  | [required] |

### Return type

**String**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_global_default_suggestions_configs_default_suggestions_post

> Vec<models::PromptSuggestion> set_global_default_suggestions_configs_default_suggestions_post(set_default_suggestions_form)
Set Global Default Suggestions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_default_suggestions_form** | [**SetDefaultSuggestionsForm**](SetDefaultSuggestionsForm.md) |  | [required] |

### Return type

[**Vec<models::PromptSuggestion>**](PromptSuggestion.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

