# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_pipeline_api_pipelines_add_post**](DefaultApi.md#add_pipeline_api_pipelines_add_post) | **POST** /api/pipelines/add | Add Pipeline
[**chat_action_api_chat_actions_action_id_post**](DefaultApi.md#chat_action_api_chat_actions_action_id_post) | **POST** /api/chat/actions/{action_id} | Chat Action
[**chat_completed_api_chat_completed_post**](DefaultApi.md#chat_completed_api_chat_completed_post) | **POST** /api/chat/completed | Chat Completed
[**delete_pipeline_api_pipelines_delete_delete**](DefaultApi.md#delete_pipeline_api_pipelines_delete_delete) | **DELETE** /api/pipelines/delete | Delete Pipeline
[**generate_chat_completions_api_chat_completions_post**](DefaultApi.md#generate_chat_completions_api_chat_completions_post) | **POST** /api/chat/completions | Generate Chat Completions
[**generate_chat_tags_api_task_tags_completions_post**](DefaultApi.md#generate_chat_tags_api_task_tags_completions_post) | **POST** /api/task/tags/completions | Generate Chat Tags
[**generate_emoji_api_task_emoji_completions_post**](DefaultApi.md#generate_emoji_api_task_emoji_completions_post) | **POST** /api/task/emoji/completions | Generate Emoji
[**generate_moa_response_api_task_moa_completions_post**](DefaultApi.md#generate_moa_response_api_task_moa_completions_post) | **POST** /api/task/moa/completions | Generate Moa Response
[**generate_search_query_api_task_query_completions_post**](DefaultApi.md#generate_search_query_api_task_query_completions_post) | **POST** /api/task/query/completions | Generate Search Query
[**generate_title_api_task_title_completions_post**](DefaultApi.md#generate_title_api_task_title_completions_post) | **POST** /api/task/title/completions | Generate Title
[**get_app_changelog_api_changelog_get**](DefaultApi.md#get_app_changelog_api_changelog_get) | **GET** /api/changelog | Get App Changelog
[**get_app_config_api_config_get**](DefaultApi.md#get_app_config_api_config_get) | **GET** /api/config | Get App Config
[**get_app_latest_release_version_api_version_updates_get**](DefaultApi.md#get_app_latest_release_version_api_version_updates_get) | **GET** /api/version/updates | Get App Latest Release Version
[**get_app_version_api_version_get**](DefaultApi.md#get_app_version_api_version_get) | **GET** /api/version | Get App Version
[**get_manifest_json_manifest_json_get**](DefaultApi.md#get_manifest_json_manifest_json_get) | **GET** /manifest.json | Get Manifest Json
[**get_model_filter_config_api_config_model_filter_get**](DefaultApi.md#get_model_filter_config_api_config_model_filter_get) | **GET** /api/config/model/filter | Get Model Filter Config
[**get_models_api_models_get**](DefaultApi.md#get_models_api_models_get) | **GET** /api/models | Get Models
[**get_opensearch_xml_opensearch_xml_get**](DefaultApi.md#get_opensearch_xml_opensearch_xml_get) | **GET** /opensearch.xml | Get Opensearch Xml
[**get_pipeline_valves_api_pipelines_pipeline_id_valves_get**](DefaultApi.md#get_pipeline_valves_api_pipelines_pipeline_id_valves_get) | **GET** /api/pipelines/{pipeline_id}/valves | Get Pipeline Valves
[**get_pipeline_valves_spec_api_pipelines_pipeline_id_valves_spec_get**](DefaultApi.md#get_pipeline_valves_spec_api_pipelines_pipeline_id_valves_spec_get) | **GET** /api/pipelines/{pipeline_id}/valves/spec | Get Pipeline Valves Spec
[**get_pipelines_api_pipelines_get**](DefaultApi.md#get_pipelines_api_pipelines_get) | **GET** /api/pipelines | Get Pipelines
[**get_pipelines_list_api_pipelines_list_get**](DefaultApi.md#get_pipelines_list_api_pipelines_list_get) | **GET** /api/pipelines/list | Get Pipelines List
[**get_task_config_api_task_config_get**](DefaultApi.md#get_task_config_api_task_config_get) | **GET** /api/task/config | Get Task Config
[**get_webhook_url_api_webhook_get**](DefaultApi.md#get_webhook_url_api_webhook_get) | **GET** /api/webhook | Get Webhook Url
[**healthcheck_health_get**](DefaultApi.md#healthcheck_health_get) | **GET** /health | Healthcheck
[**healthcheck_with_db_health_db_get**](DefaultApi.md#healthcheck_with_db_health_db_get) | **GET** /health/db | Healthcheck With Db
[**oauth_callback_oauth_provider_callback_get**](DefaultApi.md#oauth_callback_oauth_provider_callback_get) | **GET** /oauth/{provider}/callback | Oauth Callback
[**oauth_login_oauth_provider_login_get**](DefaultApi.md#oauth_login_oauth_provider_login_get) | **GET** /oauth/{provider}/login | Oauth Login
[**update_model_filter_config_api_config_model_filter_post**](DefaultApi.md#update_model_filter_config_api_config_model_filter_post) | **POST** /api/config/model/filter | Update Model Filter Config
[**update_pipeline_valves_api_pipelines_pipeline_id_valves_update_post**](DefaultApi.md#update_pipeline_valves_api_pipelines_pipeline_id_valves_update_post) | **POST** /api/pipelines/{pipeline_id}/valves/update | Update Pipeline Valves
[**update_task_config_api_task_config_update_post**](DefaultApi.md#update_task_config_api_task_config_update_post) | **POST** /api/task/config/update | Update Task Config
[**update_webhook_url_api_webhook_post**](DefaultApi.md#update_webhook_url_api_webhook_post) | **POST** /api/webhook | Update Webhook Url
[**upload_pipeline_api_pipelines_upload_post**](DefaultApi.md#upload_pipeline_api_pipelines_upload_post) | **POST** /api/pipelines/upload | Upload Pipeline



## add_pipeline_api_pipelines_add_post

> serde_json::Value add_pipeline_api_pipelines_add_post(add_pipeline_form)
Add Pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_pipeline_form** | [**AddPipelineForm**](AddPipelineForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_action_api_chat_actions_action_id_post

> serde_json::Value chat_action_api_chat_actions_action_id_post(action_id, body)
Chat Action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_completed_api_chat_completed_post

> serde_json::Value chat_completed_api_chat_completed_post(body)
Chat Completed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline_api_pipelines_delete_delete

> serde_json::Value delete_pipeline_api_pipelines_delete_delete(delete_pipeline_form)
Delete Pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_pipeline_form** | [**DeletePipelineForm**](DeletePipelineForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_chat_completions_api_chat_completions_post

> serde_json::Value generate_chat_completions_api_chat_completions_post(body, bypass_filter)
Generate Chat Completions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**bypass_filter** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_chat_tags_api_task_tags_completions_post

> serde_json::Value generate_chat_tags_api_task_tags_completions_post(body)
Generate Chat Tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_emoji_api_task_emoji_completions_post

> serde_json::Value generate_emoji_api_task_emoji_completions_post(body)
Generate Emoji

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_moa_response_api_task_moa_completions_post

> serde_json::Value generate_moa_response_api_task_moa_completions_post(body)
Generate Moa Response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_search_query_api_task_query_completions_post

> serde_json::Value generate_search_query_api_task_query_completions_post(body)
Generate Search Query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_title_api_task_title_completions_post

> serde_json::Value generate_title_api_task_title_completions_post(body)
Generate Title

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_changelog_api_changelog_get

> serde_json::Value get_app_changelog_api_changelog_get()
Get App Changelog

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


## get_app_config_api_config_get

> serde_json::Value get_app_config_api_config_get()
Get App Config

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


## get_app_latest_release_version_api_version_updates_get

> serde_json::Value get_app_latest_release_version_api_version_updates_get()
Get App Latest Release Version

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


## get_app_version_api_version_get

> serde_json::Value get_app_version_api_version_get()
Get App Version

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


## get_manifest_json_manifest_json_get

> serde_json::Value get_manifest_json_manifest_json_get()
Get Manifest Json

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


## get_model_filter_config_api_config_model_filter_get

> serde_json::Value get_model_filter_config_api_config_model_filter_get()
Get Model Filter Config

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


## get_models_api_models_get

> serde_json::Value get_models_api_models_get()
Get Models

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


## get_opensearch_xml_opensearch_xml_get

> serde_json::Value get_opensearch_xml_opensearch_xml_get()
Get Opensearch Xml

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


## get_pipeline_valves_api_pipelines_pipeline_id_valves_get

> serde_json::Value get_pipeline_valves_api_pipelines_pipeline_id_valves_get(pipeline_id, url_idx)
Get Pipeline Valves

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |
**url_idx** | Option<**i32**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_valves_spec_api_pipelines_pipeline_id_valves_spec_get

> serde_json::Value get_pipeline_valves_spec_api_pipelines_pipeline_id_valves_spec_get(pipeline_id, url_idx)
Get Pipeline Valves Spec

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |
**url_idx** | Option<**i32**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipelines_api_pipelines_get

> serde_json::Value get_pipelines_api_pipelines_get(url_idx)
Get Pipelines

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


## get_pipelines_list_api_pipelines_list_get

> serde_json::Value get_pipelines_list_api_pipelines_list_get()
Get Pipelines List

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


## get_task_config_api_task_config_get

> serde_json::Value get_task_config_api_task_config_get()
Get Task Config

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


## get_webhook_url_api_webhook_get

> serde_json::Value get_webhook_url_api_webhook_get()
Get Webhook Url

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


## healthcheck_health_get

> serde_json::Value healthcheck_health_get()
Healthcheck

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


## healthcheck_with_db_health_db_get

> serde_json::Value healthcheck_with_db_health_db_get()
Healthcheck With Db

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


## oauth_callback_oauth_provider_callback_get

> serde_json::Value oauth_callback_oauth_provider_callback_get(provider)
Oauth Callback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_login_oauth_provider_login_get

> serde_json::Value oauth_login_oauth_provider_login_get(provider)
Oauth Login

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_model_filter_config_api_config_model_filter_post

> serde_json::Value update_model_filter_config_api_config_model_filter_post(model_filter_config_form)
Update Model Filter Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_filter_config_form** | [**ModelFilterConfigForm**](ModelFilterConfigForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline_valves_api_pipelines_pipeline_id_valves_update_post

> serde_json::Value update_pipeline_valves_api_pipelines_pipeline_id_valves_update_post(pipeline_id, url_idx, body)
Update Pipeline Valves

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |
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


## update_task_config_api_task_config_update_post

> serde_json::Value update_task_config_api_task_config_update_post(task_config_form)
Update Task Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_config_form** | [**TaskConfigForm**](TaskConfigForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_url_api_webhook_post

> serde_json::Value update_webhook_url_api_webhook_post(url_form)
Update Webhook Url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_form** | [**UrlForm**](UrlForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_pipeline_api_pipelines_upload_post

> serde_json::Value upload_pipeline_api_pipelines_upload_post(url_idx, file)
Upload Pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_idx** | **i32** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

