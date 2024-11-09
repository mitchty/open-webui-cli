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

use super::LazyError;

pub async fn file(
    conf: webui::apis::configuration::Configuration,
    file: &str,
) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(file);
    let post = upload_file_files_post(&conf, path.to_path_buf()).await?;
    println!("{}", post.id);

    Ok(())
}

//  TODO right word for this?
pub async fn collection(
    conf: webui::apis::configuration::Configuration,
    form: KnowledgeForm,
) -> Result<(), Box<dyn Error>> {
    let post = create_new_knowledge_knowledge_create_post(&conf, form).await?;
    println!("{}", post.id);

    Ok(())
}
