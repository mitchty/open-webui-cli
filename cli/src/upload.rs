use std::error::Error;

use webui::apis::files_api::upload_file_files_post;

pub async fn file(
    conf: webui::apis::configuration::Configuration,
    file: &str,
) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(file);
    let post = upload_file_files_post(&conf, path.to_path_buf()).await?;
    println!("{}", post.id);

    Ok(())
}
