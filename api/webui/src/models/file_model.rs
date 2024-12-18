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
pub struct FileModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(
        rename = "hash",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hash: Option<Option<String>>,
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(
        rename = "path",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<Option<String>>,
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
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<i32>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<i32>,
}

impl FileModel {
    pub fn new(
        id: String,
        user_id: String,
        filename: String,
        created_at: Option<i32>,
        updated_at: Option<i32>,
    ) -> FileModel {
        FileModel {
            id,
            user_id,
            hash: None,
            filename,
            path: None,
            data: None,
            meta: None,
            created_at,
            updated_at,
        }
    }
}
