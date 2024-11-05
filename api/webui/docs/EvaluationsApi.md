# \EvaluationsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feedback_evaluations_feedback_post**](EvaluationsApi.md#create_feedback_evaluations_feedback_post) | **POST** /evaluations/feedback | Create Feedback
[**delete_all_feedbacks_evaluations_feedbacks_all_delete**](EvaluationsApi.md#delete_all_feedbacks_evaluations_feedbacks_all_delete) | **DELETE** /evaluations/feedbacks/all | Delete All Feedbacks
[**delete_feedback_by_id_evaluations_feedback_id_delete**](EvaluationsApi.md#delete_feedback_by_id_evaluations_feedback_id_delete) | **DELETE** /evaluations/feedback/{id} | Delete Feedback By Id
[**delete_feedbacks_evaluations_feedbacks_delete**](EvaluationsApi.md#delete_feedbacks_evaluations_feedbacks_delete) | **DELETE** /evaluations/feedbacks | Delete Feedbacks
[**get_all_feedbacks_evaluations_feedbacks_all_export_get**](EvaluationsApi.md#get_all_feedbacks_evaluations_feedbacks_all_export_get) | **GET** /evaluations/feedbacks/all/export | Get All Feedbacks
[**get_all_feedbacks_evaluations_feedbacks_all_get**](EvaluationsApi.md#get_all_feedbacks_evaluations_feedbacks_all_get) | **GET** /evaluations/feedbacks/all | Get All Feedbacks
[**get_config_evaluations_config_get**](EvaluationsApi.md#get_config_evaluations_config_get) | **GET** /evaluations/config | Get Config
[**get_feedback_by_id_evaluations_feedback_id_get**](EvaluationsApi.md#get_feedback_by_id_evaluations_feedback_id_get) | **GET** /evaluations/feedback/{id} | Get Feedback By Id
[**get_feedbacks_evaluations_feedbacks_user_get**](EvaluationsApi.md#get_feedbacks_evaluations_feedbacks_user_get) | **GET** /evaluations/feedbacks/user | Get Feedbacks
[**update_config_evaluations_config_post**](EvaluationsApi.md#update_config_evaluations_config_post) | **POST** /evaluations/config | Update Config
[**update_feedback_by_id_evaluations_feedback_id_post**](EvaluationsApi.md#update_feedback_by_id_evaluations_feedback_id_post) | **POST** /evaluations/feedback/{id} | Update Feedback By Id



## create_feedback_evaluations_feedback_post

> models::FeedbackModel create_feedback_evaluations_feedback_post(feedback_form)
Create Feedback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_form** | [**FeedbackForm**](FeedbackForm.md) |  | [required] |

### Return type

[**models::FeedbackModel**](FeedbackModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_feedbacks_evaluations_feedbacks_all_delete

> serde_json::Value delete_all_feedbacks_evaluations_feedbacks_all_delete()
Delete All Feedbacks

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


## delete_feedback_by_id_evaluations_feedback_id_delete

> serde_json::Value delete_feedback_by_id_evaluations_feedback_id_delete(id)
Delete Feedback By Id

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


## delete_feedbacks_evaluations_feedbacks_delete

> bool delete_feedbacks_evaluations_feedbacks_delete()
Delete Feedbacks

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


## get_all_feedbacks_evaluations_feedbacks_all_export_get

> Vec<models::FeedbackModel> get_all_feedbacks_evaluations_feedbacks_all_export_get()
Get All Feedbacks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FeedbackModel>**](FeedbackModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_feedbacks_evaluations_feedbacks_all_get

> Vec<models::FeedbackUserResponse> get_all_feedbacks_evaluations_feedbacks_all_get()
Get All Feedbacks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FeedbackUserResponse>**](FeedbackUserResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_evaluations_config_get

> serde_json::Value get_config_evaluations_config_get()
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


## get_feedback_by_id_evaluations_feedback_id_get

> models::FeedbackModel get_feedback_by_id_evaluations_feedback_id_get(id)
Get Feedback By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FeedbackModel**](FeedbackModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feedbacks_evaluations_feedbacks_user_get

> Vec<models::FeedbackUserResponse> get_feedbacks_evaluations_feedbacks_user_get()
Get Feedbacks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FeedbackUserResponse>**](FeedbackUserResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_evaluations_config_post

> serde_json::Value update_config_evaluations_config_post(update_config_form)
Update Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_config_form** | [**UpdateConfigForm**](UpdateConfigForm.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feedback_by_id_evaluations_feedback_id_post

> models::FeedbackModel update_feedback_by_id_evaluations_feedback_id_post(id, feedback_form)
Update Feedback By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**feedback_form** | [**FeedbackForm**](FeedbackForm.md) |  | [required] |

### Return type

[**models::FeedbackModel**](FeedbackModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

