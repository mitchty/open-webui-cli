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
pub struct ProcessTextForm {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "collection_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<Option<String>>,
}

impl ProcessTextForm {
    pub fn new(name: String, content: String) -> ProcessTextForm {
        ProcessTextForm {
            name,
            content,
            collection_name: None,
        }
    }
}
