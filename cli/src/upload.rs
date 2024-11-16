use std::error::Error;

use webui::apis::files_api::upload_file_files_post;

pub async fn file(
    conf: webui::apis::configuration::Configuration,
    file: &Vec<String>,
    collection_id: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut fids: Vec<String> = Vec::new();

    for f in file.iter() {
        let path = std::path::Path::new(f);
        let post = upload_file_files_post(&conf, path.to_path_buf()).await?;
        println!("{}", post.id);
        fids.push(post.id);
    }

    // link everything at the end if asked, if it fails the users got the file
    // ids to work with.
    //
    // TODO I think at this point these functions are getting to where they need
    // tracing/debug support in some way. Brain up a way to do all that sanely.
    if let Some(id) = collection_id {
        crate::link::collection(conf, &id, &fids).await?;
    }

    Ok(())
}
