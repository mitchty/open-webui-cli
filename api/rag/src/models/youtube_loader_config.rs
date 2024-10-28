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
pub struct YoutubeLoaderConfig {
    #[serde(rename = "language")]
    pub language: Vec<String>,
    #[serde(
        rename = "translation",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub translation: Option<Option<String>>,
}

impl YoutubeLoaderConfig {
    pub fn new(language: Vec<String>) -> YoutubeLoaderConfig {
        YoutubeLoaderConfig {
            language,
            translation: None,
        }
    }
}
