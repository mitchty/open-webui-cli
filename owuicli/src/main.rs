use std::error::Error;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::*;

use ollama::{
    apis::{
        configuration::Configuration,
        default_api::{generate_completion_api_generate_post, get_openai_models_v1_models_get},
    },
    models::{generate_completion_form, GenerateCompletionForm},
};

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct LlmArgs {
    verbose: Option<bool>,

    #[command(subcommand)]
    subcommand: Llmcommands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Llm related commands
    Llm(LlmArgs), // I have no idea what switches I need yet
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct QueryArgs {
    #[arg(short, long)]
    model: String,

    #[arg(short, long)]
    prompt: String,
}

#[derive(Subcommand, Debug)]
enum Llmcommands {
    List,
    Query(QueryArgs),
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
    let apiconf = Configuration::default();

    let conf = Configuration {
        base_path: format!("http://{}:{}{}", uri, port,apiconf.base_path),
        bearer_access_token: Some(token),
	user_agent: Some("open-webui-cli/rust".to_owned()),
        ..Configuration::default()
    };

    match &cli.command {
        Commands::Llm(sc) => match &sc.subcommand {
            Llmcommands::List => {
                if let Ok(thing) = get_openai_models_v1_models_get(&conf, None).await {
                    //		    println!("dbg: {:?}", thing);

                    let test: ModelVec = serde_json::from_value(thing.clone())?;

                    for t in test.data.iter() {
//                        println!("dbg: {:?}", t);
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
                let query = generate_completion_api_generate_post(&conf, form, None).await;
                if let Ok(query) = query {
                    let reply: PromptData = serde_json::from_value(query.clone())?;
                    println!("{}", &reply.response);
                } else {
                    println!("prompt failed: {:?}", query);
                }
            }
        },
    };

    Ok(())
}
