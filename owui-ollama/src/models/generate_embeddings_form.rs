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
pub struct GenerateEmbeddingsForm {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub options: Option<Option<serde_json::Value>>,
    #[serde(rename = "keep_alive", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<Option<Box<models::KeepAlive>>>,
}

impl GenerateEmbeddingsForm {
    pub fn new(model: String, prompt: String) -> GenerateEmbeddingsForm {
        GenerateEmbeddingsForm {
            model,
            prompt,
            options: None,
            keep_alive: None,
        }
    }
}

