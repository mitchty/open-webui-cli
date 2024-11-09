use std::error::Error;

use serde::{Deserialize, Serialize};

use webui::apis::{
    files_api::list_files_files_get, knowledge_api::get_knowledge_items_knowledge_get,
};

use ollama::apis::default_api::get_openai_models_v1_models_get;

pub async fn files(conf: webui::apis::configuration::Configuration) -> Result<(), Box<dyn Error>> {
    let values = list_files_files_get(&conf).await?;
    for i in values.iter() {
        println!(
            "id {} hash {} filename {}",
            i.id,
            i.hash.clone().unwrap_or("unknown".to_string()),
            i.meta
                .name
                .clone()
                .unwrap_or("no uploaded filename".to_string())
        );
    }
    Ok(())
}

pub async fn models(
    conf: ollama::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let thing = get_openai_models_v1_models_get(&conf, None).await?;
    let test: ModelVec = serde_json::from_value(thing.clone())?;

    for t in test.data.iter() {
        println!("{}", t.id);
    }

    Ok(())
}

// TODO should these be pub? brain on it a bit
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelVec {
    data: Vec<ModelData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelData {
    id: String,
}

pub async fn collections(
    conf: webui::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    if let Some(cs) = get_knowledge_items_knowledge_get(&conf, None).await? {
        for c in cs.iter() {
            println!("id {} name {} description {}", c.id, c.name, c.description);
        }
    }
    Ok(())
}
