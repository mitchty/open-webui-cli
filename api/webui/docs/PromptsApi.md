# \PromptsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_prompt_prompts_create_post**](PromptsApi.md#create_new_prompt_prompts_create_post) | **POST** /prompts/create | Create New Prompt
[**delete_prompt_by_command_prompts_command_command_delete_delete**](PromptsApi.md#delete_prompt_by_command_prompts_command_command_delete_delete) | **DELETE** /prompts/command/{command}/delete | Delete Prompt By Command
[**get_prompt_by_command_prompts_command_command_get**](PromptsApi.md#get_prompt_by_command_prompts_command_command_get) | **GET** /prompts/command/{command} | Get Prompt By Command
[**get_prompts_prompts_get**](PromptsApi.md#get_prompts_prompts_get) | **GET** /prompts/ | Get Prompts
[**update_prompt_by_command_prompts_command_command_update_post**](PromptsApi.md#update_prompt_by_command_prompts_command_command_update_post) | **POST** /prompts/command/{command}/update | Update Prompt By Command



## create_new_prompt_prompts_create_post

> models::PromptModel create_new_prompt_prompts_create_post(prompt_form)
Create New Prompt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_form** | [**PromptForm**](PromptForm.md) |  | [required] |

### Return type

[**models::PromptModel**](PromptModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_prompt_by_command_prompts_command_command_delete_delete

> bool delete_prompt_by_command_prompts_command_command_delete_delete(command)
Delete Prompt By Command

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_prompt_by_command_prompts_command_command_get

> models::PromptModel get_prompt_by_command_prompts_command_command_get(command)
Get Prompt By Command

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command** | **String** |  | [required] |

### Return type

[**models::PromptModel**](PromptModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_prompts_prompts_get

> Vec<models::PromptModel> get_prompts_prompts_get()
Get Prompts

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PromptModel>**](PromptModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_prompt_by_command_prompts_command_command_update_post

> models::PromptModel update_prompt_by_command_prompts_command_command_update_post(command, prompt_form)
Update Prompt By Command

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command** | **String** |  | [required] |
**prompt_form** | [**PromptForm**](PromptForm.md) |  | [required] |

### Return type

[**models::PromptModel**](PromptModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

