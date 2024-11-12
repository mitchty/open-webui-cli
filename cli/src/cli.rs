// Basically imports/sets up clap and dispatches stuff to the appropriate area
// of internal lib.

use anyhow::Result;

// Only public entry is the run function really. Everythign else is black box
// crap I will likely be changing at some point in the future.
use clap::{Parser, Subcommand};

use webui::models::{KnowledgeFileIdForm, KnowledgeForm};

// All the actual async fn calls are in these castles. module names == top level
// cli command, fn == subcommand name so future me can fix this in post more
// easily.
use crate::LazyError;

// TODO logging setup etc to log more data rather than dumping crap to stdout
// like an 80s hack.

// TODO If this becomes a library this hack is totally wrong.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let api_token = super::get_val("TOKEN", cli.token, None);
    let api_uri = super::get_val("URI", cli.uri, Some("localhost".to_string()));
    let api_port = super::get_val("PORT", cli.port, Some("8080".to_string())); // bit redundant but eh

    // TODO better error handling...
    if api_token.is_none() {
        return Err(Box::new(LazyError::new("missing api_token")));
    }

    if api_uri.is_none() {
        return Err(Box::new(LazyError::new("missing api_uri")));
    }

    if api_port.is_none() {
        return Err(Box::new(LazyError::new("missing api_port")));
    }

    // Forgive my .clone() sins cause for this abusing the heap is whatever.
    // let rag_conf = rag_conf(
    //     &api_uri.clone().unwrap(),
    //     &api_port.clone().unwrap(),
    //     &api_token.clone().unwrap(),
    // );
    let webui_conf = super::webui_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );
    let ollama_conf = super::ollama_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );
    let default_conf = super::default_conf(
        &api_uri.clone().unwrap(),
        &api_port.clone().unwrap(),
        &api_token.clone().unwrap(),
    );

    match &cli.command {
        Commands::Chat(rest) => {
            super::chat::chat(
                &rest.model,
                &rest.prompt,
                &rest.system.clone().unwrap_or_else(|| "".to_string()),
                rest.collection.clone(),
                rest.file.clone(),
                default_conf,
            )
            .await?
        }
        Commands::List(sc) => match &sc.subcommand {
            ListCommands::Models => super::list::models(ollama_conf).await?,
            ListCommands::Files => super::list::files(webui_conf).await?,
            ListCommands::Collections => super::list::collections(webui_conf).await?,
        },
        Commands::Delete(sc) => match &sc.subcommand {
            DeleteCommands::Collection(rest) => {
                super::delete::collection(&rest.id, webui_conf).await?
            }
            DeleteCommands::File(rest) => super::delete::file(&rest.id, webui_conf).await?,
            DeleteCommands::Model(rest) => super::delete::model(&rest.name, ollama_conf).await?,
        },
        Commands::New(sc) => match &sc.subcommand {
            NewCommands::Collection(rest) => {
                super::new::collection(
                    webui_conf,
                    KnowledgeForm::new(rest.name.clone(), rest.description.clone()),
                )
                .await?
            }
        },
        Commands::Link(sc) => match &sc.subcommand {
            LinkCommands::Collection(rest) => {
                super::link::collection(
                    webui_conf,
                    &rest.id,
                    KnowledgeFileIdForm::new(rest.file_id.clone()),
                )
                .await?
            }
        },
        Commands::Pull(sc) => match &sc.subcommand {
            PullCommands::Model(rest) => super::pull::model(ollama_conf, &rest.name).await?,
        },
        Commands::Upload(sc) => match &sc.subcommand {
            UploadCommands::File(rest) => super::upload::file(webui_conf, &rest.name).await?,
        },
        // Commands::Llm(sc) => match &sc.subcommand {
        //     Llmcommands::Query(rest) => query(&rest.model, &rest.prompt, ollama_conf).await?,
    };
    Ok(())
}

// Top level struct for cli command args
// TODO this is incomplete, just snagging what is needed/most useful
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    token: Option<String>,

    #[arg(short, long)]
    uri: Option<String>,

    #[arg(short, long)]
    port: Option<String>,

    // NYI what "verbose" looks like, placeholder for later
    // #[arg(short, long)]
    // verbose: Option<bool>,
    #[command(subcommand)]
    command: Commands,
}

// Top level commands
// Note: Chat has no subcommands (yet?)
#[derive(Subcommand, Debug)]
enum Commands {
    /// Chat with an llm
    Chat(ChatArgs),
    /// Delete objects
    Delete(DeleteArgs),
    /// Link objects together
    Link(LinkArgs),
    /// List objects
    List(ListArgs),
    /// Operations to create objects
    New(NewArgs),
    /// Operations to pull data
    Pull(PullArgs),
    /// Operations to upload data
    Upload(UploadArgs),
}

// open-webui chat related stuff (TODO keep ollama prompt?)
#[derive(Parser, Debug)]
struct ChatArgs {
    /// Llm model name note must include the tag e.g. name:tag
    #[arg(short, long)]
    model: String,

    /// User prompt(s) to query the llm with
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    prompt: Vec<String>,

    /// System prompt to query the llm with
    #[arg(short, long)]
    system: Option<String>,

    /// Collection id in RAG to use in prompt
    #[arg(short, long)]
    collection: Option<String>,

    /// File id in RAG to use in prompt
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Parser, Debug)]
struct ListArgs {
    #[command(subcommand)]
    subcommand: ListCommands,
}

#[derive(Subcommand, Debug)]
enum ListCommands {
    /// List knowledge collections
    Collections,
    /// List uploaded files in RAG db
    Files,
    /// List downloaded llm models
    Models,
}

// Delete related args/subcommands

#[derive(Parser, Debug)]
struct DeleteArgs {
    #[command(subcommand)]
    subcommand: DeleteCommands,
}

#[derive(Subcommand, Debug)]
enum DeleteCommands {
    /// Delete knowledge collection
    Collection(CollectionDeleteArgs),
    /// Delete files from RAG db
    File(FileDeleteArgs),
    /// Delete an llm model
    Model(ModelDeleteArgs),
}

#[derive(Parser, Debug)]
struct CollectionDeleteArgs {
    /// Collection id to delete
    #[arg(short, long)]
    id: String,
}

#[derive(Parser, Debug)]
struct FileDeleteArgs {
    /// File id to delete
    #[arg(short, long)]
    id: String,
}

#[derive(Parser, Debug)]
struct ModelDeleteArgs {
    /// Model name to delete
    #[arg(short, long)]
    name: String,
}

// New obect related args/subcommands
#[derive(Parser, Debug)]
struct NewArgs {
    #[command(subcommand)]
    subcommand: NewCommands,
}

#[derive(Subcommand, Debug)]
enum NewCommands {
    /// Create a knowledge collection
    Collection(CollectionNewArgs),
}

#[derive(Parser, Debug)]
struct CollectionNewArgs {
    /// Name of the new knowlege collection
    #[arg(short, long)]
    name: String,

    /// Description of the collection
    #[arg(short, long)]
    description: String,
}

#[derive(Parser, Debug)]
struct LinkArgs {
    #[command(subcommand)]
    subcommand: LinkCommands,
}

#[derive(Subcommand, Debug)]
enum LinkCommands {
    /// Link a file to a Collection
    Collection(LinkCollectionArgs),
}

#[derive(Parser, Debug)]
struct LinkCollectionArgs {
    /// Collection id to link file to
    #[arg(short, long)]
    id: String,

    /// File id to link into the collection
    #[arg(short, long)]
    file_id: String,
}

// New obect related args/subcommands
#[derive(Parser, Debug)]
struct PullArgs {
    #[command(subcommand)]
    subcommand: PullCommands,
}

#[derive(Subcommand, Debug)]
enum PullCommands {
    /// Pull a model to ollama instance
    Model(ModelPullArgs),
}

#[derive(Parser, Debug)]
struct ModelPullArgs {
    /// Model name to load, note should be form of model:tag, if tag is missing :latest will be assumed
    #[arg(short, long)]
    name: String,
}

// New obect related args/subcommands
#[derive(Parser, Debug)]
struct UploadArgs {
    #[command(subcommand)]
    subcommand: UploadCommands,
}

#[derive(Subcommand, Debug)]
enum UploadCommands {
    /// Upload local file to RAG db
    File(FileUploadArgs),
}

#[derive(Parser, Debug)]
struct FileUploadArgs {
    /// Local file name to upload
    #[arg(short, long)]
    name: String,
    // In future make adding to a collection easy
    // collection: Option<String>
}
