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
pub struct KnowledgeResponseFilesInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "meta")]
    pub meta: serde_json::Value,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
}

impl KnowledgeResponseFilesInner {
    pub fn new(
        id: String,
        meta: serde_json::Value,
        created_at: i32,
        updated_at: i32,
    ) -> KnowledgeResponseFilesInner {
        KnowledgeResponseFilesInner {
            id,
            meta,
            created_at,
            updated_at,
        }
    }
}