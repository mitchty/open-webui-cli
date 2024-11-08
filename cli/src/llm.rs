use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use ollama::{
    apis::default_api::{generate_completion_api_generate_post, get_openai_models_v1_models_get},
    models::GenerateCompletionForm,
};

use default::apis::default_api::generate_chat_completions_api_chat_completions_post;

use super::cli::LazyError;

// TODO should these be pub? brain on it a bit
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelVec {
    data: Vec<ModelData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelData {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PromptData {
    response: String,
}

pub async fn list(conf: ollama::apis::configuration::Configuration) -> Result<(), Box<dyn Error>> {
    if let Ok(thing) = get_openai_models_v1_models_get(&conf, None).await {
        let test: ModelVec = serde_json::from_value(thing.clone())?;

        for t in test.data.iter() {
            println!("{}", t.id);
        }
    } else {
        return Err(Box::new(LazyError::new(&format!(
            "configuration not valid {:?}",
            conf
        ))));
    }
    Ok(())
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct ChatCompletionData {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<ChatFile>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatFile {
    #[serde(rename = "type")]
    ftype: String,
    id: String,
}

// Why there are two different chat apis is beyond me
pub async fn chat(
    model: &str,
    prompt: &str,
    collection: Option<String>, // TODO vec of collections at some point
    files: Option<String>,
    conf: default::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: prompt.to_string(),
    }];

    let mut outfiles = Vec::new();

    let mut cols = None;

    if let Some(c) = collection {
        outfiles.push(ChatFile {
            ftype: "collection".to_string(),
            id: c.clone(),
        });
    }

    if let Some(f) = files {
        outfiles.push(ChatFile {
            ftype: "file".to_string(),
            id: f.clone(),
        });
    }

    if !outfiles.is_empty() {
        cols = Some(outfiles);
    }

    let body = ChatCompletionData {
        model: model.to_string(),
        messages,
        files: cols,
    };

    let http_body = serde_json::to_value(&body)?;

    let query = generate_chat_completions_api_chat_completions_post(&conf, http_body, None).await?;
    let reply: ChatData = serde_json::from_value(query.clone())?;

    for t in reply.choices.iter() {
        println!("{}", t.message.content);
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatData {
    choices: Vec<ChatMessageData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessageData {
    message: ChatMessageContentData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessageContentData {
    content: String,
}

// TODO should be split apart from this chungus file
pub async fn query(
    model: &str,
    prompt: &str,
    conf: ollama::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let form = GenerateCompletionForm {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: Some(Some(false)),
        ..GenerateCompletionForm::default()
    };
    let query = generate_completion_api_generate_post(&conf, form, None).await?;
    let reply: PromptData = serde_json::from_value(query.clone())?;
    println!("{}", &reply.response);

    Ok(())
}
