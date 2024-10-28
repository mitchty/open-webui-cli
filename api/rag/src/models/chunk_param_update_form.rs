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
pub struct ChunkParamUpdateForm {
    #[serde(rename = "chunk_size")]
    pub chunk_size: i32,
    #[serde(rename = "chunk_overlap")]
    pub chunk_overlap: i32,
}

impl ChunkParamUpdateForm {
    pub fn new(chunk_size: i32, chunk_overlap: i32) -> ChunkParamUpdateForm {
        ChunkParamUpdateForm {
            chunk_size,
            chunk_overlap,
        }
    }
}
