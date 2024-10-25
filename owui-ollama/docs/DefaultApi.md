# \DefaultApi

All URIs are relative to */ollama*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_model_api_copy_post**](DefaultApi.md#copy_model_api_copy_post) | **POST** /api/copy | Copy Model
[**copy_model_api_copy_url_idx_post**](DefaultApi.md#copy_model_api_copy_url_idx_post) | **POST** /api/copy/{url_idx} | Copy Model
[**create_model_api_create_post**](DefaultApi.md#create_model_api_create_post) | **POST** /api/create | Create Model
[**create_model_api_create_url_idx_post**](DefaultApi.md#create_model_api_create_url_idx_post) | **POST** /api/create/{url_idx} | Create Model
[**delete_model_api_delete_delete**](DefaultApi.md#delete_model_api_delete_delete) | **DELETE** /api/delete | Delete Model
[**delete_model_api_delete_url_idx_delete**](DefaultApi.md#delete_model_api_delete_url_idx_delete) | **DELETE** /api/delete/{url_idx} | Delete Model
[**download_model_models_download_post**](DefaultApi.md#download_model_models_download_post) | **POST** /models/download | Download Model
[**download_model_models_download_url_idx_post**](DefaultApi.md#download_model_models_download_url_idx_post) | **POST** /models/download/{url_idx} | Download Model
[**generate_chat_completion_api_chat_post**](DefaultApi.md#generate_chat_completion_api_chat_post) | **POST** /api/chat | Generate Chat Completion
[**generate_chat_completion_api_chat_url_idx_post**](DefaultApi.md#generate_chat_completion_api_chat_url_idx_post) | **POST** /api/chat/{url_idx} | Generate Chat Completion
[**generate_completion_api_generate_post**](DefaultApi.md#generate_completion_api_generate_post) | **POST** /api/generate | Generate Completion
[**generate_completion_api_generate_url_idx_post**](DefaultApi.md#generate_completion_api_generate_url_idx_post) | **POST** /api/generate/{url_idx} | Generate Completion
[**generate_embeddings_api_embed_post**](DefaultApi.md#generate_embeddings_api_embed_post) | **POST** /api/embed | Generate Embeddings
[**generate_embeddings_api_embed_url_idx_post**](DefaultApi.md#generate_embeddings_api_embed_url_idx_post) | **POST** /api/embed/{url_idx} | Generate Embeddings
[**generate_embeddings_api_embeddings_post**](DefaultApi.md#generate_embeddings_api_embeddings_post) | **POST** /api/embeddings | Generate Embeddings
[**generate_embeddings_api_embeddings_url_idx_post**](DefaultApi.md#generate_embeddings_api_embeddings_url_idx_post) | **POST** /api/embeddings/{url_idx} | Generate Embeddings
[**generate_openai_chat_completion_v1_chat_completions_post**](DefaultApi.md#generate_openai_chat_completion_v1_chat_completions_post) | **POST** /v1/chat/completions | Generate Openai Chat Completion
[**generate_openai_chat_completion_v1_chat_completions_url_idx_post**](DefaultApi.md#generate_openai_chat_completion_v1_chat_completions_url_idx_post) | **POST** /v1/chat/completions/{url_idx} | Generate Openai Chat Completion
[**get_config_config_get**](DefaultApi.md#get_config_config_get) | **GET** /config | Get Config
[**get_ollama_api_urls_urls_get**](DefaultApi.md#get_ollama_api_urls_urls_get) | **GET** /urls | Get Ollama Api Urls
[**get_ollama_tags_api_tags_get**](DefaultApi.md#get_ollama_tags_api_tags_get) | **GET** /api/tags | Get Ollama Tags
[**get_ollama_tags_api_tags_url_idx_get**](DefaultApi.md#get_ollama_tags_api_tags_url_idx_get) | **GET** /api/tags/{url_idx} | Get Ollama Tags
[**get_ollama_versions_api_version_get**](DefaultApi.md#get_ollama_versions_api_version_get) | **GET** /api/version | Get Ollama Versions
[**get_ollama_versions_api_version_url_idx_get**](DefaultApi.md#get_ollama_versions_api_version_url_idx_get) | **GET** /api/version/{url_idx} | Get Ollama Versions
[**get_openai_models_v1_models_get**](DefaultApi.md#get_openai_models_v1_models_get) | **GET** /v1/models | Get Openai Models
[**get_openai_models_v1_models_url_idx_get**](DefaultApi.md#get_openai_models_v1_models_url_idx_get) | **GET** /v1/models/{url_idx} | Get Openai Models
[**get_status_get**](DefaultApi.md#get_status_get) | **GET** / | Get Status
[**get_status_head**](DefaultApi.md#get_status_head) | **HEAD** / | Get Status
[**pull_model_api_pull_post**](DefaultApi.md#pull_model_api_pull_post) | **POST** /api/pull | Pull Model
[**pull_model_api_pull_url_idx_post**](DefaultApi.md#pull_model_api_pull_url_idx_post) | **POST** /api/pull/{url_idx} | Pull Model
[**push_model_api_push_delete**](DefaultApi.md#push_model_api_push_delete) | **DELETE** /api/push | Push Model
[**push_model_api_push_url_idx_delete**](DefaultApi.md#push_model_api_push_url_idx_delete) | **DELETE** /api/push/{url_idx} | Push Model
[**show_model_info_api_show_post**](DefaultApi.md#show_model_info_api_show_post) | **POST** /api/show | Show Model Info
[**update_config_config_update_post**](DefaultApi.md#update_config_config_update_post) | **POST** /config/update | Update Config
[**update_ollama_api_url_urls_update_post**](DefaultApi.md#update_ollama_api_url_urls_update_post) | **POST** /urls/update | Update Ollama Api Url
[**upload_model_models_upload_post**](DefaultApi.md#upload_model_models_upload_post) | **POST** /models/upload | Upload Model
[**upload_model_models_upload_url_idx_post**](DefaultApi.md#upload_model_models_upload_url_idx_post) | **POST** /models/upload/{url_idx} | Upload Model



## copy_model_api_copy_post

> serde_json::Value copy_model_api_copy_post(copy_model_form, url_idx)
Copy Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**copy_model_form** | [**CopyModelForm**](CopyModelForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_model_api_copy_url_idx_post

> serde_json::Value copy_model_api_copy_url_idx_post(url_idx, copy_model_form)
Copy Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**copy_model_form** | [**CopyModelForm**](CopyModelForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_model_api_create_post

> serde_json::Value create_model_api_create_post(create_model_form, url_idx)
Create Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_model_form** | [**CreateModelForm**](CreateModelForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_model_api_create_url_idx_post

> serde_json::Value create_model_api_create_url_idx_post(url_idx, create_model_form)
Create Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | **i32** |  | [required] |
**create_model_form** | [**CreateModelForm**](CreateModelForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_model_api_delete_delete

> serde_json::Value delete_model_api_delete_delete(model_name_form, url_idx)
Delete Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_name_form** | [**ModelNameForm**](ModelNameForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_model_api_delete_url_idx_delete

> serde_json::Value delete_model_api_delete_url_idx_delete(url_idx, model_name_form)
Delete Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**model_name_form** | [**ModelNameForm**](ModelNameForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_model_models_download_post

> serde_json::Value download_model_models_download_post(url_form, url_idx)
Download Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_form** | [**UrlForm**](UrlForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_model_models_download_url_idx_post

> serde_json::Value download_model_models_download_url_idx_post(url_idx, url_form)
Download Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**url_form** | [**UrlForm**](UrlForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_chat_completion_api_chat_post

> serde_json::Value generate_chat_completion_api_chat_post(generate_chat_completion_form, url_idx)
Generate Chat Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_chat_completion_form** | [**GenerateChatCompletionForm**](GenerateChatCompletionForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_chat_completion_api_chat_url_idx_post

> serde_json::Value generate_chat_completion_api_chat_url_idx_post(url_idx, generate_chat_completion_form)
Generate Chat Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**generate_chat_completion_form** | [**GenerateChatCompletionForm**](GenerateChatCompletionForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_completion_api_generate_post

> serde_json::Value generate_completion_api_generate_post(generate_completion_form, url_idx)
Generate Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_completion_form** | [**GenerateCompletionForm**](GenerateCompletionForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_completion_api_generate_url_idx_post

> serde_json::Value generate_completion_api_generate_url_idx_post(url_idx, generate_completion_form)
Generate Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**generate_completion_form** | [**GenerateCompletionForm**](GenerateCompletionForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_embeddings_api_embed_post

> serde_json::Value generate_embeddings_api_embed_post(generate_embed_form, url_idx)
Generate Embeddings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_embed_form** | [**GenerateEmbedForm**](GenerateEmbedForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_embeddings_api_embed_url_idx_post

> serde_json::Value generate_embeddings_api_embed_url_idx_post(url_idx, generate_embed_form)
Generate Embeddings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**generate_embed_form** | [**GenerateEmbedForm**](GenerateEmbedForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_embeddings_api_embeddings_post

> serde_json::Value generate_embeddings_api_embeddings_post(generate_embeddings_form, url_idx)
Generate Embeddings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_embeddings_form** | [**GenerateEmbeddingsForm**](GenerateEmbeddingsForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_embeddings_api_embeddings_url_idx_post

> serde_json::Value generate_embeddings_api_embeddings_url_idx_post(url_idx, generate_embeddings_form)
Generate Embeddings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**generate_embeddings_form** | [**GenerateEmbeddingsForm**](GenerateEmbeddingsForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_openai_chat_completion_v1_chat_completions_post

> serde_json::Value generate_openai_chat_completion_v1_chat_completions_post(body, url_idx)
Generate Openai Chat Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_openai_chat_completion_v1_chat_completions_url_idx_post

> serde_json::Value generate_openai_chat_completion_v1_chat_completions_url_idx_post(url_idx, body)
Generate Openai Chat Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_config_get

> serde_json::Value get_config_config_get()
Get Config

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


## get_ollama_api_urls_urls_get

> serde_json::Value get_ollama_api_urls_urls_get()
Get Ollama Api Urls

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


## get_ollama_tags_api_tags_get

> serde_json::Value get_ollama_tags_api_tags_get(url_idx)
Get Ollama Tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ollama_tags_api_tags_url_idx_get

> serde_json::Value get_ollama_tags_api_tags_url_idx_get(url_idx)
Get Ollama Tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ollama_versions_api_version_get

> serde_json::Value get_ollama_versions_api_version_get(url_idx)
Get Ollama Versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ollama_versions_api_version_url_idx_get

> serde_json::Value get_ollama_versions_api_version_url_idx_get(url_idx)
Get Ollama Versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_openai_models_v1_models_get

> serde_json::Value get_openai_models_v1_models_get(url_idx)
Get Openai Models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_openai_models_v1_models_url_idx_get

> serde_json::Value get_openai_models_v1_models_url_idx_get(url_idx)
Get Openai Models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status_get

> serde_json::Value get_status_get()
Get Status

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


## get_status_head

> serde_json::Value get_status_head()
Get Status

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


## pull_model_api_pull_post

> serde_json::Value pull_model_api_pull_post(model_name_form, url_idx)
Pull Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_name_form** | [**ModelNameForm**](ModelNameForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_model_api_pull_url_idx_post

> serde_json::Value pull_model_api_pull_url_idx_post(url_idx, model_name_form)
Pull Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | **i32** |  | [required] |
**model_name_form** | [**ModelNameForm**](ModelNameForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_model_api_push_delete

> serde_json::Value push_model_api_push_delete(push_model_form, url_idx)
Push Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_model_form** | [**PushModelForm**](PushModelForm.md) |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_model_api_push_url_idx_delete

> serde_json::Value push_model_api_push_url_idx_delete(url_idx, push_model_form)
Push Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**push_model_form** | [**PushModelForm**](PushModelForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_model_info_api_show_post

> serde_json::Value show_model_info_api_show_post(model_name_form)
Show Model Info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_name_form** | [**ModelNameForm**](ModelNameForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_config_update_post

> serde_json::Value update_config_config_update_post(ollama_config_form)
Update Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ollama_config_form** | [**OllamaConfigForm**](OllamaConfigForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ollama_api_url_urls_update_post

> serde_json::Value update_ollama_api_url_urls_update_post(url_update_form)
Update Ollama Api Url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_update_form** | [**UrlUpdateForm**](UrlUpdateForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_model_models_upload_post

> serde_json::Value upload_model_models_upload_post(file, url_idx)
Upload Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** |  | [required] |
**url_idx** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_model_models_upload_url_idx_post

> serde_json::Value upload_model_models_upload_url_idx_post(url_idx, file)
Upload Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | Option<**i32**> |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

