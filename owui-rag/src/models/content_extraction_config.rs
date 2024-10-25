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
pub struct ContentExtractionConfig {
    #[serde(rename = "engine", skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "tika_server_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tika_server_url: Option<Option<String>>,
}

impl ContentExtractionConfig {
    pub fn new() -> ContentExtractionConfig {
        ContentExtractionConfig {
            engine: None,
            tika_server_url: None,
        }
    }
}
