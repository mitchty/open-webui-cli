use std::error::Error;

use webui::{
    apis::knowledge_api::add_file_to_knowledge_by_id_knowledge_id_file_add_post,
    models::KnowledgeFileIdForm,
};

// TODO here too...
pub async fn collection(
    conf: webui::apis::configuration::Configuration,
    id: &str,
    file_id: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for i in file_id.iter() {
        let form = KnowledgeFileIdForm::new(i.to_owned());
        let _post = add_file_to_knowledge_by_id_knowledge_id_file_add_post(&conf, id, form).await?;
    }
    Ok(())
}
