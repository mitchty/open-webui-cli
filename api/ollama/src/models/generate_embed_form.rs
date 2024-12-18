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
pub struct GenerateEmbedForm {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "input")]
    pub input: Box<models::Input>,
    #[serde(
        rename = "truncate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub truncate: Option<Option<bool>>,
    #[serde(
        rename = "options",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<Option<serde_json::Value>>,
    #[serde(
        rename = "keep_alive",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_alive: Option<Option<Box<models::KeepAlive>>>,
}

impl GenerateEmbedForm {
    pub fn new(model: String, input: models::Input) -> GenerateEmbedForm {
        GenerateEmbedForm {
            model,
            input: Box::new(input),
            truncate: None,
            options: None,
            keep_alive: None,
        }
    }
}
