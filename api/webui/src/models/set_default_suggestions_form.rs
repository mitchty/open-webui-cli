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
pub struct SetDefaultSuggestionsForm {
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<models::PromptSuggestion>,
}

impl SetDefaultSuggestionsForm {
    pub fn new(suggestions: Vec<models::PromptSuggestion>) -> SetDefaultSuggestionsForm {
        SetDefaultSuggestionsForm { suggestions }
    }
}