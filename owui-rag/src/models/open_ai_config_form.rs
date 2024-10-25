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
pub struct OpenAiConfigForm {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "batch_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<Option<i32>>,
}

impl OpenAiConfigForm {
    pub fn new(url: String, key: String) -> OpenAiConfigForm {
        OpenAiConfigForm {
            url,
            key,
            batch_size: None,
        }
    }
}

