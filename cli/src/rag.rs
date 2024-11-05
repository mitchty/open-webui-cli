use std::error::Error;

use serde::{Deserialize, Serialize};

use webui::apis::files_api::upload_file_files_post;

use super::cli::LazyError;
use std::path::PathBuf;

pub async fn upload(
    conf: webui::apis::configuration::Configuration,
    file: PathBuf,
) -> Result<(), Box<dyn Error>> {
    if let Ok(post) = upload_file_files_post(&conf, file).await {
        println!("{}", post.id);
    } else {
        return Err(LazyError::new(&format!("post failed {:?}", conf)).into());
    }
    Ok(())
}
