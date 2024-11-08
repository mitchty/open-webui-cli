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
pub struct FolderIsExpandedForm {
    #[serde(rename = "is_expanded")]
    pub is_expanded: bool,
}

impl FolderIsExpandedForm {
    pub fn new(is_expanded: bool) -> FolderIsExpandedForm {
        FolderIsExpandedForm { is_expanded }
    }
}