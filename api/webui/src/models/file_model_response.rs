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
pub struct FileModelResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "meta")]
    pub meta: models::FileMeta,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
}

impl FileModelResponse {
    pub fn new(
        id: String,
        user_id: String,
        filename: String,
        meta: models::FileMeta,
        created_at: i32,
        updated_at: i32,
    ) -> FileModelResponse {
        FileModelResponse {
            id,
            user_id,
            hash: None,
            filename,
            meta,
            created_at,
            updated_at,
        }
    }
}
