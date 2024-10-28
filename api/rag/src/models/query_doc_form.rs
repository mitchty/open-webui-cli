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
pub struct QueryDocForm {
    #[serde(rename = "collection_name")]
    pub collection_name: String,
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "k", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub k: Option<Option<i32>>,
    #[serde(rename = "r", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r: Option<Option<f64>>,
    #[serde(rename = "hybrid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<Option<bool>>,
}

impl QueryDocForm {
    pub fn new(collection_name: String, query: String) -> QueryDocForm {
        QueryDocForm {
            collection_name,
            query,
            k: None,
            r: None,
            hybrid: None,
        }
    }
}
