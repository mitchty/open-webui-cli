use std::error::Error;

use serde::{Deserialize, Serialize};

use default::apis::default_api::get_app_config_api_config_get;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigData {
    name: String,
    version: String,
}

// For now just abuse the /api/config endpoint for data
pub async fn config(
    uri: &str,
    conf: default::apis::configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let post = get_app_config_api_config_get(&conf).await?;

    let reply: ConfigData = serde_json::from_value(post)?;

    println!("{} at {} version {} ", reply.name, uri, reply.version);
    Ok(())
}
