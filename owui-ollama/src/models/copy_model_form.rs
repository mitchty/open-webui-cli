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
pub struct CopyModelForm {
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "destination")]
    pub destination: String,
}

impl CopyModelForm {
    pub fn new(source: String, destination: String) -> CopyModelForm {
        CopyModelForm {
            source,
            destination,
        }
    }
}

