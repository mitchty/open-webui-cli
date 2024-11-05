# \ChatsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_tag_by_id_and_tag_name_chats_id_tags_post**](ChatsApi.md#add_tag_by_id_and_tag_name_chats_id_tags_post) | **POST** /chats/{id}/tags | Add Tag By Id And Tag Name
[**archive_all_chats_chats_archive_all_post**](ChatsApi.md#archive_all_chats_chats_archive_all_post) | **POST** /chats/archive/all | Archive All Chats
[**archive_chat_by_id_chats_id_archive_post**](ChatsApi.md#archive_chat_by_id_chats_id_archive_post) | **POST** /chats/{id}/archive | Archive Chat By Id
[**clone_chat_by_id_chats_id_clone_post**](ChatsApi.md#clone_chat_by_id_chats_id_clone_post) | **POST** /chats/{id}/clone | Clone Chat By Id
[**create_new_chat_chats_new_post**](ChatsApi.md#create_new_chat_chats_new_post) | **POST** /chats/new | Create New Chat
[**delete_all_tags_by_id_chats_id_tags_all_delete**](ChatsApi.md#delete_all_tags_by_id_chats_id_tags_all_delete) | **DELETE** /chats/{id}/tags/all | Delete All Tags By Id
[**delete_all_user_chats_chats_delete**](ChatsApi.md#delete_all_user_chats_chats_delete) | **DELETE** /chats/ | Delete All User Chats
[**delete_chat_by_id_chats_id_delete**](ChatsApi.md#delete_chat_by_id_chats_id_delete) | **DELETE** /chats/{id} | Delete Chat By Id
[**delete_shared_chat_by_id_chats_id_share_delete**](ChatsApi.md#delete_shared_chat_by_id_chats_id_share_delete) | **DELETE** /chats/{id}/share | Delete Shared Chat By Id
[**delete_tag_by_id_and_tag_name_chats_id_tags_delete**](ChatsApi.md#delete_tag_by_id_and_tag_name_chats_id_tags_delete) | **DELETE** /chats/{id}/tags | Delete Tag By Id And Tag Name
[**get_all_user_chats_in_db_chats_all_db_get**](ChatsApi.md#get_all_user_chats_in_db_chats_all_db_get) | **GET** /chats/all/db | Get All User Chats In Db
[**get_all_user_tags_chats_all_tags_get**](ChatsApi.md#get_all_user_tags_chats_all_tags_get) | **GET** /chats/all/tags | Get All User Tags
[**get_archived_session_user_chat_list_chats_archived_get**](ChatsApi.md#get_archived_session_user_chat_list_chats_archived_get) | **GET** /chats/archived | Get Archived Session User Chat List
[**get_chat_by_id_chats_id_get**](ChatsApi.md#get_chat_by_id_chats_id_get) | **GET** /chats/{id} | Get Chat By Id
[**get_chat_tags_by_id_chats_id_tags_get**](ChatsApi.md#get_chat_tags_by_id_chats_id_tags_get) | **GET** /chats/{id}/tags | Get Chat Tags By Id
[**get_chats_by_folder_id_chats_folder_folder_id_get**](ChatsApi.md#get_chats_by_folder_id_chats_folder_folder_id_get) | **GET** /chats/folder/{folder_id} | Get Chats By Folder Id
[**get_pinned_status_by_id_chats_id_pinned_get**](ChatsApi.md#get_pinned_status_by_id_chats_id_pinned_get) | **GET** /chats/{id}/pinned | Get Pinned Status By Id
[**get_session_user_chat_list_chats_get**](ChatsApi.md#get_session_user_chat_list_chats_get) | **GET** /chats/ | Get Session User Chat List
[**get_session_user_chat_list_chats_list_get**](ChatsApi.md#get_session_user_chat_list_chats_list_get) | **GET** /chats/list | Get Session User Chat List
[**get_shared_chat_by_id_chats_share_share_id_get**](ChatsApi.md#get_shared_chat_by_id_chats_share_share_id_get) | **GET** /chats/share/{share_id} | Get Shared Chat By Id
[**get_user_archived_chats_chats_all_archived_get**](ChatsApi.md#get_user_archived_chats_chats_all_archived_get) | **GET** /chats/all/archived | Get User Archived Chats
[**get_user_chat_list_by_tag_name_chats_tags_post**](ChatsApi.md#get_user_chat_list_by_tag_name_chats_tags_post) | **POST** /chats/tags | Get User Chat List By Tag Name
[**get_user_chat_list_by_user_id_chats_list_user_user_id_get**](ChatsApi.md#get_user_chat_list_by_user_id_chats_list_user_user_id_get) | **GET** /chats/list/user/{user_id} | Get User Chat List By User Id
[**get_user_chats_chats_all_get**](ChatsApi.md#get_user_chats_chats_all_get) | **GET** /chats/all | Get User Chats
[**get_user_pinned_chats_chats_pinned_get**](ChatsApi.md#get_user_pinned_chats_chats_pinned_get) | **GET** /chats/pinned | Get User Pinned Chats
[**import_chat_chats_import_post**](ChatsApi.md#import_chat_chats_import_post) | **POST** /chats/import | Import Chat
[**pin_chat_by_id_chats_id_pin_post**](ChatsApi.md#pin_chat_by_id_chats_id_pin_post) | **POST** /chats/{id}/pin | Pin Chat By Id
[**search_user_chats_chats_search_get**](ChatsApi.md#search_user_chats_chats_search_get) | **GET** /chats/search | Search User Chats
[**share_chat_by_id_chats_id_share_post**](ChatsApi.md#share_chat_by_id_chats_id_share_post) | **POST** /chats/{id}/share | Share Chat By Id
[**update_chat_by_id_chats_id_post**](ChatsApi.md#update_chat_by_id_chats_id_post) | **POST** /chats/{id} | Update Chat By Id
[**update_chat_folder_id_by_id_chats_id_folder_post**](ChatsApi.md#update_chat_folder_id_by_id_chats_id_folder_post) | **POST** /chats/{id}/folder | Update Chat Folder Id By Id



## add_tag_by_id_and_tag_name_chats_id_tags_post

> Vec<models::TagModel> add_tag_by_id_and_tag_name_chats_id_tags_post(id, tag_form)
Add Tag By Id And Tag Name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tag_form** | [**TagForm**](TagForm.md) |  | [required] |

### Return type

[**Vec<models::TagModel>**](TagModel.md)

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


## archive_chat_by_id_chats_id_archive_post

> models::ChatResponse archive_chat_by_id_chats_id_archive_post(id)
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


## clone_chat_by_id_chats_id_clone_post

> models::ChatResponse clone_chat_by_id_chats_id_clone_post(id)
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

> models::ChatResponse create_new_chat_chats_new_post(chat_form)
Create New Chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_form** | [**ChatForm**](ChatForm.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_tags_by_id_chats_id_tags_all_delete

> bool delete_all_tags_by_id_chats_id_tags_all_delete(id)
Delete All Tags By Id

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


## delete_tag_by_id_and_tag_name_chats_id_tags_delete

> Vec<models::TagModel> delete_tag_by_id_and_tag_name_chats_id_tags_delete(id, tag_form)
Delete Tag By Id And Tag Name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tag_form** | [**TagForm**](TagForm.md) |  | [required] |

### Return type

[**Vec<models::TagModel>**](TagModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
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


## get_all_user_tags_chats_all_tags_get

> Vec<models::TagModel> get_all_user_tags_chats_all_tags_get()
Get All User Tags

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


## get_chats_by_folder_id_chats_folder_folder_id_get

> Vec<models::ChatResponse> get_chats_by_folder_id_chats_folder_folder_id_get(folder_id)
Get Chats By Folder Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** |  | [required] |

### Return type

[**Vec<models::ChatResponse>**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pinned_status_by_id_chats_id_pinned_get

> bool get_pinned_status_by_id_chats_id_pinned_get(id)
Get Pinned Status By Id

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

> Vec<models::ChatTitleIdResponse> get_user_chat_list_by_tag_name_chats_tags_post(tag_filter_form)
Get User Chat List By Tag Name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_filter_form** | [**TagFilterForm**](TagFilterForm.md) |  | [required] |

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


## get_user_pinned_chats_chats_pinned_get

> Vec<models::ChatResponse> get_user_pinned_chats_chats_pinned_get()
Get User Pinned Chats

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


## import_chat_chats_import_post

> models::ChatResponse import_chat_chats_import_post(chat_import_form)
Import Chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_import_form** | [**ChatImportForm**](ChatImportForm.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_chat_by_id_chats_id_pin_post

> models::ChatResponse pin_chat_by_id_chats_id_pin_post(id)
Pin Chat By Id

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


## search_user_chats_chats_search_get

> Vec<models::ChatTitleIdResponse> search_user_chats_chats_search_get(text, page)
Search User Chats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::ChatTitleIdResponse>**](ChatTitleIdResponse.md)

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

> models::ChatResponse update_chat_by_id_chats_id_post(id, chat_form)
Update Chat By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_form** | [**ChatForm**](ChatForm.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_folder_id_by_id_chats_id_folder_post

> models::ChatResponse update_chat_folder_id_by_id_chats_id_folder_post(id, chat_folder_id_form)
Update Chat Folder Id By Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_folder_id_form** | [**ChatFolderIdForm**](ChatFolderIdForm.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

