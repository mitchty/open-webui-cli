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
pub struct FunctionResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "meta")]
    pub meta: Box<models::FunctionMeta>,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "is_global")]
    pub is_global: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
    #[serde(rename = "created_at")]
    pub created_at: i32,
}

impl FunctionResponse {
    pub fn new(
        id: String,
        user_id: String,
        r#type: String,
        name: String,
        meta: models::FunctionMeta,
        is_active: bool,
        is_global: bool,
        updated_at: i32,
        created_at: i32,
    ) -> FunctionResponse {
        FunctionResponse {
            id,
            user_id,
            r#type,
            name,
            meta: Box::new(meta),
            is_active,
            is_global,
            updated_at,
            created_at,
        }
    }
}
