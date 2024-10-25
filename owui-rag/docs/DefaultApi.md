# \DefaultApi

All URIs are relative to */retrieval/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_entries_from_collection_delete_post**](DefaultApi.md#delete_entries_from_collection_delete_post) | **POST** /delete | Delete Entries From Collection
[**get_embedding_config_embedding_get**](DefaultApi.md#get_embedding_config_embedding_get) | **GET** /embedding | Get Embedding Config
[**get_embeddings_ef_get**](DefaultApi.md#get_embeddings_ef_get) | **GET** /ef | Get Embeddings
[**get_embeddings_text_ef_text_get**](DefaultApi.md#get_embeddings_text_ef_text_get) | **GET** /ef/{text} | Get Embeddings Text
[**get_query_settings_query_settings_get**](DefaultApi.md#get_query_settings_query_settings_get) | **GET** /query/settings | Get Query Settings
[**get_rag_config_config_get**](DefaultApi.md#get_rag_config_config_get) | **GET** /config | Get Rag Config
[**get_rag_template_template_get**](DefaultApi.md#get_rag_template_template_get) | **GET** /template | Get Rag Template
[**get_reraanking_config_reranking_get**](DefaultApi.md#get_reraanking_config_reranking_get) | **GET** /reranking | Get Reraanking Config
[**get_status_get**](DefaultApi.md#get_status_get) | **GET** / | Get Status
[**process_file_process_file_post**](DefaultApi.md#process_file_process_file_post) | **POST** /process/file | Process File
[**process_text_process_text_post**](DefaultApi.md#process_text_process_text_post) | **POST** /process/text | Process Text
[**process_web_process_web_post**](DefaultApi.md#process_web_process_web_post) | **POST** /process/web | Process Web
[**process_web_search_process_web_search_post**](DefaultApi.md#process_web_search_process_web_search_post) | **POST** /process/web/search | Process Web Search
[**process_youtube_video_process_youtube_post**](DefaultApi.md#process_youtube_video_process_youtube_post) | **POST** /process/youtube | Process Youtube Video
[**query_collection_handler_query_collection_post**](DefaultApi.md#query_collection_handler_query_collection_post) | **POST** /query/collection | Query Collection Handler
[**query_doc_handler_query_doc_post**](DefaultApi.md#query_doc_handler_query_doc_post) | **POST** /query/doc | Query Doc Handler
[**reset_reset_post**](DefaultApi.md#reset_reset_post) | **POST** /reset | Reset
[**reset_upload_dir_reset_uploads_post**](DefaultApi.md#reset_upload_dir_reset_uploads_post) | **POST** /reset/uploads | Reset Upload Dir
[**reset_vector_db_reset_db_post**](DefaultApi.md#reset_vector_db_reset_db_post) | **POST** /reset/db | Reset Vector Db
[**update_embedding_config_embedding_update_post**](DefaultApi.md#update_embedding_config_embedding_update_post) | **POST** /embedding/update | Update Embedding Config
[**update_query_settings_query_settings_update_post**](DefaultApi.md#update_query_settings_query_settings_update_post) | **POST** /query/settings/update | Update Query Settings
[**update_rag_config_config_update_post**](DefaultApi.md#update_rag_config_config_update_post) | **POST** /config/update | Update Rag Config
[**update_reranking_config_reranking_update_post**](DefaultApi.md#update_reranking_config_reranking_update_post) | **POST** /reranking/update | Update Reranking Config



## delete_entries_from_collection_delete_post

> serde_json::Value delete_entries_from_collection_delete_post(delete_form)
Delete Entries From Collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_form** | [**DeleteForm**](DeleteForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_embedding_config_embedding_get

> serde_json::Value get_embedding_config_embedding_get()
Get Embedding Config

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


## get_embeddings_ef_get

> serde_json::Value get_embeddings_ef_get()
Get Embeddings

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


## get_embeddings_text_ef_text_get

> serde_json::Value get_embeddings_text_ef_text_get(text)
Get Embeddings Text

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_query_settings_query_settings_get

> serde_json::Value get_query_settings_query_settings_get()
Get Query Settings

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


## get_rag_config_config_get

> serde_json::Value get_rag_config_config_get()
Get Rag Config

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


## get_rag_template_template_get

> serde_json::Value get_rag_template_template_get()
Get Rag Template

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


## get_reraanking_config_reranking_get

> serde_json::Value get_reraanking_config_reranking_get()
Get Reraanking Config

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


## process_file_process_file_post

> serde_json::Value process_file_process_file_post(process_file_form)
Process File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**process_file_form** | [**ProcessFileForm**](ProcessFileForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_text_process_text_post

> serde_json::Value process_text_process_text_post(process_text_form)
Process Text

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**process_text_form** | [**ProcessTextForm**](ProcessTextForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_web_process_web_post

> serde_json::Value process_web_process_web_post(process_url_form)
Process Web

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**process_url_form** | [**ProcessUrlForm**](ProcessUrlForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_web_search_process_web_search_post

> serde_json::Value process_web_search_process_web_search_post(search_form)
Process Web Search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_form** | [**SearchForm**](SearchForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_youtube_video_process_youtube_post

> serde_json::Value process_youtube_video_process_youtube_post(process_url_form)
Process Youtube Video

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**process_url_form** | [**ProcessUrlForm**](ProcessUrlForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_collection_handler_query_collection_post

> serde_json::Value query_collection_handler_query_collection_post(query_collections_form)
Query Collection Handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_collections_form** | [**QueryCollectionsForm**](QueryCollectionsForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_doc_handler_query_doc_post

> serde_json::Value query_doc_handler_query_doc_post(query_doc_form)
Query Doc Handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_doc_form** | [**QueryDocForm**](QueryDocForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_reset_post

> bool reset_reset_post()
Reset

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_upload_dir_reset_uploads_post

> bool reset_upload_dir_reset_uploads_post()
Reset Upload Dir

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_vector_db_reset_db_post

> serde_json::Value reset_vector_db_reset_db_post()
Reset Vector Db

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


## update_embedding_config_embedding_update_post

> serde_json::Value update_embedding_config_embedding_update_post(embedding_model_update_form)
Update Embedding Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**embedding_model_update_form** | [**EmbeddingModelUpdateForm**](EmbeddingModelUpdateForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_query_settings_query_settings_update_post

> serde_json::Value update_query_settings_query_settings_update_post(query_settings_form)
Update Query Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_settings_form** | [**QuerySettingsForm**](QuerySettingsForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rag_config_config_update_post

> serde_json::Value update_rag_config_config_update_post(config_update_form)
Update Rag Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_update_form** | [**ConfigUpdateForm**](ConfigUpdateForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reranking_config_reranking_update_post

> serde_json::Value update_reranking_config_reranking_update_post(reranking_model_update_form)
Update Reranking Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reranking_model_update_form** | [**RerankingModelUpdateForm**](RerankingModelUpdateForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

