use std::error::Error;

use serde::{Deserialize, Serialize};

use ollama::{
    apis::default_api::{generate_completion_api_generate_post, get_openai_models_v1_models_get},
    models::GenerateCompletionForm,
};

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
    let query = generate_completion_api_generate_post(&conf, form, None).await;
    if let Ok(query) = query {
        let reply: PromptData = serde_json::from_value(query.clone())?;
        println!("{}", &reply.response);
    } else {
        return Err(Box::new(LazyError::new(&format!(
            "prompt failed: {:?}",
            query
        ))));
    }
    Ok(())
}
