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
pub struct DeletePipelineForm {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "urlIdx")]
    pub url_idx: i32,
}

impl DeletePipelineForm {
    pub fn new(id: String, url_idx: i32) -> DeletePipelineForm {
        DeletePipelineForm { id, url_idx }
    }
}
