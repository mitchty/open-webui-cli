use std::error::Error;

use webui::{
    apis::{
        files_api::upload_file_files_post,
        knowledge_api::{
            add_file_to_knowledge_by_id_knowledge_id_file_add_post,
            create_new_knowledge_knowledge_create_post,
        },
    },
    models::{KnowledgeFileIdForm, KnowledgeForm},
};

use super::cli::LazyError;

pub async fn upload(
    conf: webui::apis::configuration::Configuration,
    file: &str,
) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(file);
    let post = upload_file_files_post(&conf, path.to_path_buf()).await?;
    println!("{}", post.id);

    Ok(())
}

pub async fn create(
    conf: webui::apis::configuration::Configuration,
    form: KnowledgeForm,
) -> Result<(), Box<dyn Error>> {
    if let Ok(post) = create_new_knowledge_knowledge_create_post(&conf, form).await {
        println!("{}", post.id);
    } else {
        return Err(LazyError::new(&format!("create knowledge failed {:?}", conf)).into());
    }
    Ok(())
}

pub async fn add(
    conf: webui::apis::configuration::Configuration,
    id: &str,
    form: KnowledgeFileIdForm,
) -> Result<(), Box<dyn Error>> {
    let post = add_file_to_knowledge_by_id_knowledge_id_file_add_post(&conf, &id, form).await?;
    println!("{}", post.id);
    Ok(())
}
