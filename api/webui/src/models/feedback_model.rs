/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedbackModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(
        rename = "data",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<Option<serde_json::Value>>,
    #[serde(
        rename = "meta",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<Option<serde_json::Value>>,
    #[serde(
        rename = "snapshot",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot: Option<Option<serde_json::Value>>,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
}

impl FeedbackModel {
    pub fn new(
        id: String,
        user_id: String,
        version: i32,
        r#type: String,
        created_at: i32,
        updated_at: i32,
    ) -> FeedbackModel {
        FeedbackModel {
            id,
            user_id,
            version,
            r#type,
            data: None,
            meta: None,
            snapshot: None,
            created_at,
            updated_at,
        }
    }
}