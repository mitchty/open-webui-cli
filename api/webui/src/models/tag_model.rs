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
pub struct TagModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(
        rename = "meta",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<Option<serde_json::Value>>,
}

impl TagModel {
    pub fn new(id: String, name: String, user_id: String) -> TagModel {
        TagModel {
            id,
            name,
            user_id,
            meta: None,
        }
    }
}
