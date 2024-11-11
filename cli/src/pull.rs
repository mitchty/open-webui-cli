use std::error::Error;

use serde::{Deserialize, Serialize};

use ollama::{
    apis::default_api::{get_openai_models_v1_models_get, pull_model_api_pull_post},
    models::ModelNameForm,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    status: String,
}

pub async fn model(
    conf: ollama::apis::configuration::Configuration,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    let form = ModelNameForm::new(name.to_string());
    let post = pull_model_api_pull_post(&conf, form, None).await?;
    let status: Status = serde_json::from_value(post.clone())?;

    // The api is kinda wonky, if you don't do a GET on the models after posting
    // it can't be deleted, so after the post we do a get on everything and
    // ignore the response.
    if status.status == "success" {
        let _get = get_openai_models_v1_models_get(&conf, None).await?; // maybe use .context here instead?
        Ok(())
    } else {
        Err(Box::new(crate::LazyError::new(&format!(
            "pull of model {} failed with {:?}",
            name, post
        ))))
    }
}