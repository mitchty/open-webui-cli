use std::error::Error;

use webui::apis::{
    files_api::delete_file_by_id_files_id_delete,
    knowledge_api::delete_knowledge_by_id_knowledge_id_delete_delete,
};

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
