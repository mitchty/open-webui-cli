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
pub struct BannerModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "dismissible")]
    pub dismissible: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
}

impl BannerModel {
    pub fn new(
        id: String,
        r#type: String,
        content: String,
        dismissible: bool,
        timestamp: i32,
    ) -> BannerModel {
        BannerModel {
            id,
            r#type,
            title: None,
            content,
            dismissible,
            timestamp,
        }
    }
}