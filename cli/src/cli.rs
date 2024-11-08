// Basically imports/sets up clap and dispatches stuff to the appropriate area
// of internal lib.

use anyhow::Result;

// Only public entry is the run function really. Everythign else is black box
// crap I will likely be changing at some point in the future.
use clap::{Parser, Subcommand};

use std::fmt;
use webui::models::{KnowledgeFileIdForm, KnowledgeForm};

// All the actual async fn calls are in these castles. module names == top level
// cli command, fn == subcommand name so future me can fix this in post more
// easily.
use super::llm::*;
use super::rag::*;

// I'm lazy so just for now have a lame af error struct. Future mitch gets to fix it. Sucker....
//
// might make sense as its own module too? Top level of the lib.rs is an option too.
#[derive(Debug)]
pub struct LazyError {
    details: String,
}

impl LazyError {
    pub fn new(msg: &str) -> LazyError {
        LazyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LazyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for LazyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// TODO maybe make this crap a macro? Needs cleanup at some point all quick
// hacks to get work done for now. Just silly boilerplate so watevs not
// critical, just wasting loc really. Gated by however many of these openapi
// generated api crates that get abused in this hack of a cli.
//
// In the end who gives af, this is all just run once per cli invocation not a critical concern.j
//
// This might better be in its own module space too..
fn rag_conf(uri: &str, port: &str, token: &str) -> rag::apis::configuration::Configuration {
    let def_conf = rag::apis::configuration::Configuration::default();

    rag::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, def_conf.base_path),
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..rag::apis::configuration::Configuration::default()
    }
}

fn webui_conf(uri: &str, port: &str, token: &str) -> webui::apis::configuration::Configuration {
    let def_conf = webui::apis::configuration::Configuration::default();

    webui::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, def_conf.base_path),
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..webui::apis::configuration::Configuration::default()
    }
}

fn default_conf(uri: &str, port: &str, token: &str) -> default::apis::configuration::Configuration {
    default::apis::configuration::Configuration {
        base_path: format!("http://{}:{}", uri, port), // base_path is http://localhost here... sigh consistency is for the birds apparently
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..default::apis::configuration::Configuration::default()
    }
}

fn ollama_conf(uri: &str, port: &str, token: &str) -> ollama::apis::configuration::Configuration {
    let def_conf = ollama::apis::configuration::Configuration::default();

    ollama::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, def_conf.base_path),
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..ollama::apis::configuration::Configuration::default()
    }
}

// TODO logging setup etc to log more data rather than dumping crap to stdout
// like an 80s hack.

// TODO If this becomes a library this hack is totally wrong.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let api_token = get_val("TOKEN", cli.token, None);
    let api_uri = get_val("URI", cli.uri, Some("localhost".to_string()));
    let api_port = get_val("PORT", cli.port, Some("8080".to_string())); // bit redundant but eh

    // TODO better error handling...
    if api_token.is_none() {
        return Err(LazyError::new(&format!("missing api_token")).into());
    }

    if api_uri.is_none() {
        return Err(LazyError::new(&format!("missing api_uri")).into());
    }

    if api_port.is_none() {
        return Err(LazyError::new(&format!("missing api_port")).into());
    }

    // Forgive my .clone() sins cause for this abusing the heap is whatever.
    let rag_conf = rag_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );
    let webui_conf = webui_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );
    let ollama_conf = ollama_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );
    let default_conf = default_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );

    match &cli.command {
        Commands::Llm(sc) => match &sc.subcommand {
            Llmcommands::List => list(ollama_conf).await?,
            Llmcommands::Query(rest) => query(&rest.model, &rest.prompt, ollama_conf).await?,
            Llmcommands::Chat(rest) => {
                chat(
                    &rest.model,
                    &rest.prompt,
                    rest.collection.clone(),
                    rest.file.clone(),
                    default_conf,
                )
                .await?
            }
        },
        Commands::Rag(sc) => match &sc.subcommand {
            Ragcommands::Upload(rest) => upload(webui_conf, &rest.file.clone()).await?,
            Ragcommands::Create(rest) => {
                create(
                    webui_conf,
                    KnowledgeForm::new(rest.name.clone(), rest.description.clone()),
                )
                .await?
            }
            Ragcommands::Add(rest) => {
                add(
                    webui_conf,
                    &rest.id,
                    KnowledgeFileIdForm::new(rest.file_id.clone()),
                )
                .await?
            }
            Ragcommands::List(_none) => {
                listfiles(webui_conf).await?;
            }
            Ragcommands::Delete(rest) => {
                deletefile(&rest.id, webui_conf).await?;
            }
        },
    };
    Ok(())
}

// Order of input data is as follows:
// - config (NYI)
// - env vars
// - switches
//
// This will let us have default configs we can read out of xdg home, followed
// by env vars to superced that, with the final determination being a switch.
//
// There has gotta be a crate for something like this already. If not I should
// write one.

// Clamped to Strings for now cause past mitch is a lazy bastard, can make it
// generic later. For now only need it on strings anyway so watev.
fn get_val(env_name: &str, cli_val: Option<String>, default: Option<String>) -> Option<String> {
    // Logic here is simply inverted, pass ownership to caller
    if let Some(cli) = cli_val {
        return Some(cli.to_string());
    }

    if let Ok(val) = std::env::var(env_name) {
        return Some(val.to_string());
    }

    if let Some(val) = default {
        return Some(val.to_string());
    }

    None
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct LlmArgs {
    verbose: Option<bool>,

    #[command(subcommand)]
    subcommand: Llmcommands,
}

#[derive(Subcommand, Debug)]
enum Llmcommands {
    /// List installed llm models
    List,
    /// Query llm
    Query(QueryArgs),
    /// Chat with llm
    Chat(ChatArgs),
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
struct ChatArgs {
    #[arg(short, long)]
    model: String,

    #[arg(short, long)]
    prompt: String,

    #[arg(short, long)]
    collection: Option<String>,

    #[arg(short, long)]
    file: Option<String>,
}

// TODO this is incomplete, just snagging what I use for now

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    token: Option<String>,

    #[arg(short, long)]
    uri: Option<String>,

    #[arg(short, long)]
    port: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Llm related commands
    Llm(LlmArgs), // I have no idea what switches I need yet
    /// Rag related commands
    Rag(RagArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RagArgs {
    verbose: Option<bool>,

    #[command(subcommand)]
    subcommand: Ragcommands,
}

#[derive(Subcommand, Debug)]
enum Ragcommands {
    /// Upload a file to the rag vector db
    Upload(UploadArgs),
    /// Create a knowledge collection
    Create(CreateKnowledgeArgs),
    /// Add an already uploaded file to a collection
    Add(AddKnowledgeArgs),
    /// List uploaded files
    List(ListArgs),
    /// Delete an uploaded file
    Delete(DeleteArgs),
}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct UploadArgs {
    #[arg(short, long)]
    file: String,
    // #[arg(long)]
    // collection: Option<String>,
}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct CreateKnowledgeArgs {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    description: String,
}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct AddKnowledgeArgs {
    #[arg(short, long)]
    id: String,

    #[arg(short, long)]
    file_id: String,
}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct ListArgs {
    // Empty for now
}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct DeleteArgs {
    #[arg(short, long)]
    id: String,
}
