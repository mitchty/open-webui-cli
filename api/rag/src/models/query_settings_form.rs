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
pub struct QuerySettingsForm {
    #[serde(
        rename = "k",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub k: Option<Option<i32>>,
    #[serde(
        rename = "r",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r: Option<Option<f64>>,
    #[serde(
        rename = "template",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub template: Option<Option<String>>,
    #[serde(
        rename = "hybrid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hybrid: Option<Option<bool>>,
}

impl QuerySettingsForm {
    pub fn new() -> QuerySettingsForm {
        QuerySettingsForm {
            k: None,
            r: None,
            template: None,
            hybrid: None,
        }
    }
}
