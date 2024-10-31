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
pub struct KnowledgeFileIdForm {
    #[serde(rename = "file_id")]
    pub file_id: String,
}

impl KnowledgeFileIdForm {
    pub fn new(file_id: String) -> KnowledgeFileIdForm {
        KnowledgeFileIdForm { file_id }
    }
}