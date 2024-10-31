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
pub struct OpenWebuiAppsWebuiModelsAuthsUserResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "profile_image_url")]
    pub profile_image_url: String,
}

impl OpenWebuiAppsWebuiModelsAuthsUserResponse {
    pub fn new(
        id: String,
        email: String,
        name: String,
        role: String,
        profile_image_url: String,
    ) -> OpenWebuiAppsWebuiModelsAuthsUserResponse {
        OpenWebuiAppsWebuiModelsAuthsUserResponse {
            id,
            email,
            name,
            role,
            profile_image_url,
        }
    }
}