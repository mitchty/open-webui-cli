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
pub struct DeleteForm {
    #[serde(rename = "collection_name")]
    pub collection_name: String,
    #[serde(rename = "file_id")]
    pub file_id: String,
}

impl DeleteForm {
    pub fn new(collection_name: String, file_id: String) -> DeleteForm {
        DeleteForm {
            collection_name,
            file_id,
        }
    }
}
