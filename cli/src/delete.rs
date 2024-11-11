use std::error::Error;

use webui::apis::{
    files_api::delete_file_by_id_files_id_delete,
    knowledge_api::delete_knowledge_by_id_knowledge_id_delete_delete,
};

use ollama::{apis::default_api::delete_model_api_delete_delete, models::ModelNameForm};

pub async fn file(
    id: &str,
    conf: webui::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let _resp = delete_file_by_id_files_id_delete(&conf, id).await?;
    Ok(())
}

pub async fn collection(
    id: &str,
    conf: webui::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let _resp = delete_knowledge_by_id_knowledge_id_delete_delete(&conf, id).await?;
    Ok(())
}

pub async fn model(
    name: &str,
    conf: ollama::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let form = ModelNameForm::new(name.to_string());
    let resp = delete_model_api_delete_delete(&conf, form, None).await?;
    if resp == serde_json::Value::Bool(true) {
        Ok(())
    } else {
        // I have no idea when that api would return false
        Err(Box::new(crate::LazyError::new(&format!(
            "delete of model {} did not return true",
            name
        ))))
    }
}
