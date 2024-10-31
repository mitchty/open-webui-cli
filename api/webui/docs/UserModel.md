# UserModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**email** | **String** |  | 
**role** | Option<**String**> |  | [optional][default to pending]
**profile_image_url** | **String** |  | 
**last_active_at** | **i32** |  | 
**updated_at** | **i32** |  | 
**created_at** | **i32** |  | 
**api_key** | Option<**String**> |  | [optional]
**settings** | Option<[**models::UserSettings**](UserSettings.md)> |  | [optional]
**info** | Option<[**serde_json::Value**](.md)> |  | [optional]
**oauth_sub** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


