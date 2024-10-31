use std::error::Error;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::*;

use ollama::{
    apis::default_api::{generate_completion_api_generate_post, get_openai_models_v1_models_get},
    models::{generate_completion_form, GenerateCompletionForm},
};

use rag::{apis::default_api::process_file_process_file_post, models::ProcessFileForm};
use webui::{apis::files_api::upload_file_files_post, models::FileModel};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    token: Option<String>,

    #[arg(short, long)]
    uri: Option<String>,

    #[arg(short, long)]
    port: Option<u32>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Llm related commands
    Llm(LlmArgs), // I have no idea what switches I need yet
    Rag(RagArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct LlmArgs {
    verbose: Option<bool>,

    #[command(subcommand)]
    subcommand: Llmcommands,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RagArgs {
    verbose: Option<bool>,

    #[command(subcommand)]
    subcommand: Ragcommands,
}

#[derive(Subcommand, Debug)]
enum Llmcommands {
    List,
    Query(QueryArgs),
}

#[derive(Subcommand, Debug)]
enum Ragcommands {
    Upload(UploadArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct QueryArgs {
    #[arg(short, long)]
    model: String,

    #[arg(short, long)]
    prompt: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct UploadArgs {
    #[arg(short, long)]
    file: String,
    // #[arg(long)]
    // collection: Option<String>,
}

// TODO this is incomplete, just snagging what I use for now
#[derive(Serialize, Deserialize, Debug)]
struct ModelVec {
    data: Vec<ModelData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelData {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PromptData {
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileUploadData {
    id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO HACKS FIXME
    // cli switches take precedence over env vars, total hack atm
    let mut token = "token-unset".to_string();
    if let Some(thing) = cli.token {
        token = thing.clone();
    } else {
        token = match std::env::var("TOKEN") {
            Ok(value) => format!("{}", value).to_string(),
            Err(_) => format!("no token set").to_string(),
        };
    };

    let mut uri = "uri-unset".to_string();
    if let Some(thing) = cli.uri {
        uri = thing.clone();
    } else {
        uri = match std::env::var("URI") {
            Ok(value) => format!("{}", value).to_string(),
            Err(_) => format!("no uri set").to_string(),
        };
    };

    let mut port = "8080".to_string();
    if let Some(thing) = cli.port {
        port = format!("{}", thing).to_string();
    } else {
        port = match std::env::var("PORT") {
            Ok(value) => format!("{}", value).to_string(),
            Err(_) => format!("8080").to_string(),
        };
    };

    // The open-webui openapi generated code is a bit terrible. Get the default
    // configuration so we can get the base_path that the api has setup so we
    // can use that as a suffix to the uri/port we connect to.
    let ollama_api_conf = ollama::apis::configuration::Configuration::default();

    let llm_conf = ollama::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, ollama_api_conf.base_path),
        bearer_access_token: Some(token.clone()), // TODO fixme no .clone() lazy bastard
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..ollama::apis::configuration::Configuration::default()
    };

    let rag_api_conf = rag::apis::configuration::Configuration::default();

    let rag_conf = rag::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, rag_api_conf.base_path),
        bearer_access_token: Some(token.clone()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..rag::apis::configuration::Configuration::default()
    };

    let webui_api_conf = webui::apis::configuration::Configuration::default();

    let webui_conf = webui::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, webui_api_conf.base_path),
        bearer_access_token: Some(token),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..webui::apis::configuration::Configuration::default()
    };

    match &cli.command {
        Commands::Llm(sc) => match &sc.subcommand {
            Llmcommands::List => {
                if let Ok(thing) = get_openai_models_v1_models_get(&llm_conf, None).await {
                    let test: ModelVec = serde_json::from_value(thing.clone())?;

                    for t in test.data.iter() {
                        println!("{}", t.id);
                    }
                }
            }
            Llmcommands::Query(rest) => {
                let model = rest.model.clone();
                let prompt = rest.prompt.clone();
                let form = GenerateCompletionForm {
                    model,
                    prompt,
                    stream: Some(Some(false)),
                    ..GenerateCompletionForm::default()
                };
                let query = generate_completion_api_generate_post(&llm_conf, form, None).await;
                if let Ok(query) = query {
                    let reply: PromptData = serde_json::from_value(query.clone())?;
                    println!("{}", &reply.response);
                } else {
                    println!("prompt failed: {:?}", query);
                }
            }
        },
        Commands::Rag(sc) => match &sc.subcommand {
            Ragcommands::Upload(rest) => {
                use std::path::Path;
                let post =
                    upload_file_files_post(&webui_conf, Path::new(&rest.file).to_path_buf()).await;
                if let Ok(post) = post {
                    //                    println!("dbg: {:?}", post);
                    let reply: FileUploadData = serde_json::from_value(post.clone())?;
                    println!("{}", reply.id);
                } else {
                    println!("upload failed: {:?}", post);
                }
            }
        },
    };

    Ok(())
}
