# \AuthsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_auths_add_post**](AuthsApi.md#add_user_auths_add_post) | **POST** /auths/add | Add User
[**create_api_key_auths_api_key_post**](AuthsApi.md#create_api_key_auths_api_key_post) | **POST** /auths/api_key | Create Api Key 
[**delete_api_key_auths_api_key_delete**](AuthsApi.md#delete_api_key_auths_api_key_delete) | **DELETE** /auths/api_key | Delete Api Key
[**get_admin_config_auths_admin_config_get**](AuthsApi.md#get_admin_config_auths_admin_config_get) | **GET** /auths/admin/config | Get Admin Config
[**get_admin_details_auths_admin_details_get**](AuthsApi.md#get_admin_details_auths_admin_details_get) | **GET** /auths/admin/details | Get Admin Details
[**get_api_key_auths_api_key_get**](AuthsApi.md#get_api_key_auths_api_key_get) | **GET** /auths/api_key | Get Api Key
[**get_session_user_auths_get**](AuthsApi.md#get_session_user_auths_get) | **GET** /auths/ | Get Session User
[**signin_auths_signin_post**](AuthsApi.md#signin_auths_signin_post) | **POST** /auths/signin | Signin
[**signup_auths_signup_post**](AuthsApi.md#signup_auths_signup_post) | **POST** /auths/signup | Signup
[**update_admin_config_auths_admin_config_post**](AuthsApi.md#update_admin_config_auths_admin_config_post) | **POST** /auths/admin/config | Update Admin Config
[**update_password_auths_update_password_post**](AuthsApi.md#update_password_auths_update_password_post) | **POST** /auths/update/password | Update Password
[**update_profile_auths_update_profile_post**](AuthsApi.md#update_profile_auths_update_profile_post) | **POST** /auths/update/profile | Update Profile



## add_user_auths_add_post

> models::SigninResponse add_user_auths_add_post(add_user_form)
Add User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_user_form** | [**AddUserForm**](AddUserForm.md) |  | [required] |

### Return type

[**models::SigninResponse**](SigninResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_key_auths_api_key_post

> models::ApiKey create_api_key_auths_api_key_post()
Create Api Key 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key_auths_api_key_delete

> bool delete_api_key_auths_api_key_delete()
Delete Api Key

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


## get_admin_config_auths_admin_config_get

> serde_json::Value get_admin_config_auths_admin_config_get()
Get Admin Config

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


## get_admin_details_auths_admin_details_get

> serde_json::Value get_admin_details_auths_admin_details_get()
Get Admin Details

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


## get_api_key_auths_api_key_get

> models::ApiKey get_api_key_auths_api_key_get()
Get Api Key

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session_user_auths_get

> models::OpenWebuiAppsWebuiModelsAuthsUserResponse get_session_user_auths_get()
Get Session User

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OpenWebuiAppsWebuiModelsAuthsUserResponse**](open_webui__apps__webui__models__auths__UserResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signin_auths_signin_post

> models::SigninResponse signin_auths_signin_post(signin_form)
Signin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signin_form** | [**SigninForm**](SigninForm.md) |  | [required] |

### Return type

[**models::SigninResponse**](SigninResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signup_auths_signup_post

> models::SigninResponse signup_auths_signup_post(signup_form)
Signup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signup_form** | [**SignupForm**](SignupForm.md) |  | [required] |

### Return type

[**models::SigninResponse**](SigninResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_admin_config_auths_admin_config_post

> serde_json::Value update_admin_config_auths_admin_config_post(admin_config)
Update Admin Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin_config** | [**AdminConfig**](AdminConfig.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_password_auths_update_password_post

> bool update_password_auths_update_password_post(update_password_form)
Update Password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_password_form** | [**UpdatePasswordForm**](UpdatePasswordForm.md) |  | [required] |

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile_auths_update_profile_post

> models::OpenWebuiAppsWebuiModelsAuthsUserResponse update_profile_auths_update_profile_post(update_profile_form)
Update Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_profile_form** | [**UpdateProfileForm**](UpdateProfileForm.md) |  | [required] |

### Return type

[**models::OpenWebuiAppsWebuiModelsAuthsUserResponse**](open_webui__apps__webui__models__auths__UserResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

