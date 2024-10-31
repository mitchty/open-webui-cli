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
pub struct FunctionModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "meta")]
    pub meta: Box<models::FunctionMeta>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "is_global", skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
    #[serde(rename = "created_at")]
    pub created_at: i32,
}

impl FunctionModel {
    pub fn new(
        id: String,
        user_id: String,
        name: String,
        r#type: String,
        content: String,
        meta: models::FunctionMeta,
        updated_at: i32,
        created_at: i32,
    ) -> FunctionModel {
        FunctionModel {
            id,
            user_id,
            name,
            r#type,
            content,
            meta: Box::new(meta),
            is_active: None,
            is_global: None,
            updated_at,
            created_at,
        }
    }
}
