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
pub struct ChatForm {
    #[serde(rename = "chat")]
    pub chat: serde_json::Value,
}

impl ChatForm {
    pub fn new(chat: serde_json::Value) -> ChatForm {
        ChatForm { chat }
    }
}
