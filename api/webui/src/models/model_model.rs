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
pub struct ModelModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(
        rename = "base_model_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_model_id: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "meta")]
    pub meta: models::ModelMeta,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
    #[serde(rename = "created_at")]
    pub created_at: i32,
}

impl ModelModel {
    pub fn new(
        id: String,
        user_id: String,
        name: String,
        params: std::collections::HashMap<String, serde_json::Value>,
        meta: models::ModelMeta,
        updated_at: i32,
        created_at: i32,
    ) -> ModelModel {
        ModelModel {
            id,
            user_id,
            base_model_id: None,
            name,
            params,
            meta,
            updated_at,
            created_at,
        }
    }
}
