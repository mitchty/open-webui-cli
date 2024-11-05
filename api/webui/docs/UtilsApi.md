# \UtilsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_chat_as_pdf_utils_pdf_post**](UtilsApi.md#download_chat_as_pdf_utils_pdf_post) | **POST** /utils/pdf | Download Chat As Pdf
[**download_db_utils_db_download_get**](UtilsApi.md#download_db_utils_db_download_get) | **GET** /utils/db/download | Download Db
[**download_litellm_config_yaml_utils_litellm_config_get**](UtilsApi.md#download_litellm_config_yaml_utils_litellm_config_get) | **GET** /utils/litellm/config | Download Litellm Config Yaml
[**format_code_utils_code_format_post**](UtilsApi.md#format_code_utils_code_format_post) | **POST** /utils/code/format | Format Code
[**get_gravatar_utils_gravatar_get**](UtilsApi.md#get_gravatar_utils_gravatar_get) | **GET** /utils/gravatar | Get Gravatar
[**get_html_from_markdown_utils_markdown_post**](UtilsApi.md#get_html_from_markdown_utils_markdown_post) | **POST** /utils/markdown | Get Html From Markdown



## download_chat_as_pdf_utils_pdf_post

> serde_json::Value download_chat_as_pdf_utils_pdf_post(chat_title_messages_form)
Download Chat As Pdf

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_title_messages_form** | [**ChatTitleMessagesForm**](ChatTitleMessagesForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_db_utils_db_download_get

> serde_json::Value download_db_utils_db_download_get()
Download Db

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


## download_litellm_config_yaml_utils_litellm_config_get

> serde_json::Value download_litellm_config_yaml_utils_litellm_config_get()
Download Litellm Config Yaml

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


## format_code_utils_code_format_post

> serde_json::Value format_code_utils_code_format_post(code_format_request)
Format Code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_format_request** | [**CodeFormatRequest**](CodeFormatRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gravatar_utils_gravatar_get

> serde_json::Value get_gravatar_utils_gravatar_get(email)
Get Gravatar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_html_from_markdown_utils_markdown_post

> serde_json::Value get_html_from_markdown_utils_markdown_post(markdown_form)
Get Html From Markdown

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**markdown_form** | [**MarkdownForm**](MarkdownForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

