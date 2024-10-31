# \ChatsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_chat_tag_by_id_chats_id_tags_post**](ChatsApi.md#add_chat_tag_by_id_chats_id_tags_post) | **POST** /chats/{id}/tags | Add Chat Tag By Id
[**archive_all_chats_chats_archive_all_post**](ChatsApi.md#archive_all_chats_chats_archive_all_post) | **POST** /chats/archive/all | Archive All Chats
[**archive_chat_by_id_chats_id_archive_get**](ChatsApi.md#archive_chat_by_id_chats_id_archive_get) | **GET** /chats/{id}/archive | Archive Chat By Id
[**clone_chat_by_id_chats_id_clone_get**](ChatsApi.md#clone_chat_by_id_chats_id_clone_get) | **GET** /chats/{id}/clone | Clone Chat By Id
[**create_new_chat_chats_new_post**](ChatsApi.md#create_new_chat_chats_new_post) | **POST** /chats/new | Create New Chat
[**delete_all_chat_tags_by_id_chats_id_tags_all_delete**](ChatsApi.md#delete_all_chat_tags_by_id_chats_id_tags_all_delete) | **DELETE** /chats/{id}/tags/all | Delete All Chat Tags By Id
[**delete_all_user_chats_chats_delete**](ChatsApi.md#delete_all_user_chats_chats_delete) | **DELETE** /chats/ | Delete All User Chats
[**delete_chat_by_id_chats_id_delete**](ChatsApi.md#delete_chat_by_id_chats_id_delete) | **DELETE** /chats/{id} | Delete Chat By Id
[**delete_chat_tag_by_id_chats_id_tags_delete**](ChatsApi.md#delete_chat_tag_by_id_chats_id_tags_delete) | **DELETE** /chats/{id}/tags | Delete Chat Tag By Id
[**delete_shared_chat_by_id_chats_id_share_delete**](ChatsApi.md#delete_shared_chat_by_id_chats_id_share_delete) | **DELETE** /chats/{id}/share | Delete Shared Chat By Id
[**get_all_tags_chats_tags_all_get**](ChatsApi.md#get_all_tags_chats_tags_all_get) | **GET** /chats/tags/all | Get All Tags
[**get_all_user_chats_in_db_chats_all_db_get**](ChatsApi.md#get_all_user_chats_in_db_chats_all_db_get) | **GET** /chats/all/db | Get All User Chats In Db
[**get_archived_session_user_chat_list_chats_archived_get**](ChatsApi.md#get_archived_session_user_chat_list_chats_archived_get) | **GET** /chats/archived | Get Archived Session User Chat List
[**get_chat_by_id_chats_id_get**](ChatsApi.md#get_chat_by_id_chats_id_get) | **GET** /chats/{id} | Get Chat By Id
[**get_chat_tags_by_id_chats_id_tags_get**](ChatsApi.md#get_chat_tags_by_id_chats_id_tags_get) | **GET** /chats/{id}/tags | Get Chat Tags By Id
[**get_session_user_chat_list_chats_get**](ChatsApi.md#get_session_user_chat_list_chats_get) | **GET** /chats/ | Get Session User Chat List
[**get_session_user_chat_list_chats_list_get**](ChatsApi.md#get_session_user_chat_list_chats_list_get) | **GET** /chats/list | Get Session User Chat List
[**get_shared_chat_by_id_chats_share_share_id_get**](ChatsApi.md#get_shared_chat_by_id_chats_share_share_id_get) | **GET** /chats/share/{share_id} | Get Shared Chat By Id
[**get_user_archived_chats_chats_all_archived_get**](ChatsApi.md#get_user_archived_chats_chats_all_archived_get) | **GET** /chats/all/archived | Get User Archived Chats
[**get_user_chat_list_by_tag_name_chats_tags_post**](ChatsApi.md#get_user_chat_list_by_tag_name_chats_tags_post) | **POST** /chats/tags | Get User Chat List By Tag Name
[**get_user_chat_list_by_user_id_chats_list_user_user_id_get**](ChatsApi.md#get_user_chat_list_by_user_id_chats_list_user_user_id_get) | **GET** /chats/list/user/{user_id} | Get User Chat List By User Id
[**get_user_chats_chats_all_get**](ChatsApi.md#get_user_chats_chats_all_get) | **GET** /chats/all | Get User Chats
[**share_chat_by_id_chats_id_share_post**](ChatsApi.md#share_chat_by_id_chats_id_share_post) | **POST** /chats/{id}/share | Share Chat By Id
[**update_chat_by_id_chats_id_post**](ChatsApi.md#update_chat_by_id_chats_id_post) | **POST** /chats/{id} | Update Chat By Id



## add_chat_tag_by_id_chats_id_tags_post

> models::ChatIdTagModel add_chat_tag_by_id_chats_id_tags_post(id, chat_id_tag_form)
Add Chat Tag By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_id_tag_form** | [**ChatIdTagForm**](ChatIdTagForm.md) |  | [required] |

### Return type

[**models::ChatIdTagModel**](ChatIdTagModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_all_chats_chats_archive_all_post

> bool archive_all_chats_chats_archive_all_post()
Archive All Chats

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


## archive_chat_by_id_chats_id_archive_get

> models::ChatResponse archive_chat_by_id_chats_id_archive_get(id)
Archive Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_chat_by_id_chats_id_clone_get

> models::ChatResponse clone_chat_by_id_chats_id_clone_get(id)
Clone Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_new_chat_chats_new_post

> models::ChatResponse create_new_chat_chats_new_post(open_webui_apps_webui_models_chats_chat_form)
Create New Chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_webui_apps_webui_models_chats_chat_form** | [**OpenWebuiAppsWebuiModelsChatsChatForm**](OpenWebuiAppsWebuiModelsChatsChatForm.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_chat_tags_by_id_chats_id_tags_all_delete

> bool delete_all_chat_tags_by_id_chats_id_tags_all_delete(id)
Delete All Chat Tags By Id

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


## delete_all_user_chats_chats_delete

> bool delete_all_user_chats_chats_delete()
Delete All User Chats

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


## delete_chat_by_id_chats_id_delete

> bool delete_chat_by_id_chats_id_delete(id)
Delete Chat By Id

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


## delete_chat_tag_by_id_chats_id_tags_delete

> bool delete_chat_tag_by_id_chats_id_tags_delete(id, chat_id_tag_form)
Delete Chat Tag By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_id_tag_form** | [**ChatIdTagForm**](ChatIdTagForm.md) |  | [required] |

### Return type

**bool**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_shared_chat_by_id_chats_id_share_delete

> bool delete_shared_chat_by_id_chats_id_share_delete(id)
Delete Shared Chat By Id

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


## get_all_tags_chats_tags_all_get

> Vec<models::TagModel> get_all_tags_chats_tags_all_get()
Get All Tags

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TagModel>**](TagModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_chats_in_db_chats_all_db_get

> Vec<models::ChatResponse> get_all_user_chats_in_db_chats_all_db_get()
Get All User Chats In Db

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ChatResponse>**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_session_user_chat_list_chats_archived_get

> Vec<models::ChatTitleIdResponse> get_archived_session_user_chat_list_chats_archived_get(skip, limit)
Get Archived Session User Chat List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**Vec<models::ChatTitleIdResponse>**](ChatTitleIdResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_by_id_chats_id_get

> models::ChatResponse get_chat_by_id_chats_id_get(id)
Get Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_tags_by_id_chats_id_tags_get

> Vec<models::TagModel> get_chat_tags_by_id_chats_id_tags_get(id)
Get Chat Tags By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<models::TagModel>**](TagModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session_user_chat_list_chats_get

> Vec<models::ChatTitleIdResponse> get_session_user_chat_list_chats_get(page)
Get Session User Chat List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::ChatTitleIdResponse>**](ChatTitleIdResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session_user_chat_list_chats_list_get

> Vec<models::ChatTitleIdResponse> get_session_user_chat_list_chats_list_get(page)
Get Session User Chat List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::ChatTitleIdResponse>**](ChatTitleIdResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_chat_by_id_chats_share_share_id_get

> models::ChatResponse get_shared_chat_by_id_chats_share_share_id_get(share_id)
Get Shared Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**share_id** | **String** |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_archived_chats_chats_all_archived_get

> Vec<models::ChatResponse> get_user_archived_chats_chats_all_archived_get()
Get User Archived Chats

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ChatResponse>**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_chat_list_by_tag_name_chats_tags_post

> Vec<models::ChatTitleIdResponse> get_user_chat_list_by_tag_name_chats_tags_post(tag_name_form)
Get User Chat List By Tag Name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_name_form** | [**TagNameForm**](TagNameForm.md) |  | [required] |

### Return type

[**Vec<models::ChatTitleIdResponse>**](ChatTitleIdResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_chat_list_by_user_id_chats_list_user_user_id_get

> Vec<models::ChatTitleIdResponse> get_user_chat_list_by_user_id_chats_list_user_user_id_get(user_id, skip, limit)
Get User Chat List By User Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**Vec<models::ChatTitleIdResponse>**](ChatTitleIdResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_chats_chats_all_get

> Vec<models::ChatResponse> get_user_chats_chats_all_get()
Get User Chats

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ChatResponse>**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_chat_by_id_chats_id_share_post

> models::ChatResponse share_chat_by_id_chats_id_share_post(id)
Share Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_by_id_chats_id_post

> models::ChatResponse update_chat_by_id_chats_id_post(id, open_webui_apps_webui_models_chats_chat_form)
Update Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**open_webui_apps_webui_models_chats_chat_form** | [**OpenWebuiAppsWebuiModelsChatsChatForm**](OpenWebuiAppsWebuiModelsChatsChatForm.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

