use std::error::Error;

use webui::{
    apis::{
        files_api::{
            delete_file_by_id_files_id_delete, list_files_files_get, upload_file_files_post,
        },
        knowledge_api::{
            add_file_to_knowledge_by_id_knowledge_id_file_add_post,
            create_new_knowledge_knowledge_create_post,
        },
    },
    models::{KnowledgeFileIdForm, KnowledgeForm},
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
