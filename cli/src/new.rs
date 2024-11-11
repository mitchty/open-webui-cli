use std::error::Error;

use webui::{
    apis::knowledge_api::create_new_knowledge_knowledge_create_post, models::KnowledgeForm,
};

pub async fn collection(
    conf: webui::apis::configuration::Configuration,
    form: KnowledgeForm,
) -> Result<(), Box<dyn Error>> {
    let post = create_new_knowledge_knowledge_create_post(&conf, form).await?;
    println!("{}", post.id);
    Ok(())
}
