# \UsersApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_by_id_users_user_id_delete**](UsersApi.md#delete_user_by_id_users_user_id_delete) | **DELETE** /users/{user_id} | Delete User By Id
[**get_user_by_id_users_user_id_get**](UsersApi.md#get_user_by_id_users_user_id_get) | **GET** /users/{user_id} | Get User By Id
[**get_user_info_by_session_user_users_user_info_get**](UsersApi.md#get_user_info_by_session_user_users_user_info_get) | **GET** /users/user/info | Get User Info By Session User
[**get_user_permissions_users_permissions_user_get**](UsersApi.md#get_user_permissions_users_permissions_user_get) | **GET** /users/permissions/user | Get User Permissions
[**get_user_settings_by_session_user_users_user_settings_get**](UsersApi.md#get_user_settings_by_session_user_users_user_settings_get) | **GET** /users/user/settings | Get User Settings By Session User
[**get_users_users_get**](UsersApi.md#get_users_users_get) | **GET** /users/ | Get Users
[**update_user_by_id_users_user_id_update_post**](UsersApi.md#update_user_by_id_users_user_id_update_post) | **POST** /users/{user_id}/update | Update User By Id
[**update_user_info_by_session_user_users_user_info_update_post**](UsersApi.md#update_user_info_by_session_user_users_user_info_update_post) | **POST** /users/user/info/update | Update User Info By Session User
[**update_user_permissions_users_permissions_user_post**](UsersApi.md#update_user_permissions_users_permissions_user_post) | **POST** /users/permissions/user | Update User Permissions
[**update_user_role_users_update_role_post**](UsersApi.md#update_user_role_users_update_role_post) | **POST** /users/update/role | Update User Role
[**update_user_settings_by_session_user_users_user_settings_update_post**](UsersApi.md#update_user_settings_by_session_user_users_user_settings_update_post) | **POST** /users/user/settings/update | Update User Settings By Session User



## delete_user_by_id_users_user_id_delete

> bool delete_user_by_id_users_user_id_delete(user_id)
Delete User By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id_users_user_id_get

> models::OpenWebuiAppsWebuiRoutersUsersUserResponse get_user_by_id_users_user_id_get(user_id)
Get User By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**models::OpenWebuiAppsWebuiRoutersUsersUserResponse**](open_webui__apps__webui__routers__users__UserResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_info_by_session_user_users_user_info_get

> serde_json::Value get_user_info_by_session_user_users_user_info_get()
Get User Info By Session User

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


## get_user_permissions_users_permissions_user_get

> serde_json::Value get_user_permissions_users_permissions_user_get()
Get User Permissions

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


## get_user_settings_by_session_user_users_user_settings_get

> models::UserSettings get_user_settings_by_session_user_users_user_settings_get()
Get User Settings By Session User

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserSettings**](UserSettings.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_users_get

> Vec<models::UserModel> get_users_users_get(skip, limit)
Get Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**Vec<models::UserModel>**](UserModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_by_id_users_user_id_update_post

> models::UserModel update_user_by_id_users_user_id_update_post(user_id, user_update_form)
Update User By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**user_update_form** | [**UserUpdateForm**](UserUpdateForm.md) |  | [required] |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_info_by_session_user_users_user_info_update_post

> serde_json::Value update_user_info_by_session_user_users_user_info_update_post(body)
Update User Info By Session User

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


## update_user_permissions_users_permissions_user_post

> serde_json::Value update_user_permissions_users_permissions_user_post(body)
Update User Permissions

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


## update_user_role_users_update_role_post

> models::UserModel update_user_role_users_update_role_post(user_role_update_form)
Update User Role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_role_update_form** | [**UserRoleUpdateForm**](UserRoleUpdateForm.md) |  | [required] |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_settings_by_session_user_users_user_settings_update_post

> models::UserSettings update_user_settings_by_session_user_users_user_settings_update_post(user_settings)
Update User Settings By Session User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_settings** | [**UserSettings**](UserSettings.md) |  | [required] |

### Return type

[**models::UserSettings**](UserSettings.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

