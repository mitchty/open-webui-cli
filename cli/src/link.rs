use std::error::Error;

use webui::{
    apis::knowledge_api::add_file_to_knowledge_by_id_knowledge_id_file_add_post,
    models::KnowledgeFileIdForm,
};

// TODO here too...
pub async fn collection(
    conf: webui::apis::configuration::Configuration,
    id: &str,
    form: KnowledgeFileIdForm,
) -> Result<(), Box<dyn Error>> {
    let post = add_file_to_knowledge_by_id_knowledge_id_file_add_post(&conf, id, form).await?;
    println!("{}", post.id);
    Ok(())
}
