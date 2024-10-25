use clap::{Parser, Subcommand};
use owui_ollama::apis::configuration::Configuration;
use owui_ollama::apis::default_api::get_openai_models_v1_models_get;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    token: String,

    #[arg(short, long)]
    host: String,

    #[arg(short, long)]
    port: u32,

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

#[derive(Subcommand, Debug)]
enum Llmcommands {
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let conf = Configuration {
        base_path: format!("http://{}:{}", cli.host.clone(), cli.port),
        bearer_access_token: Some(cli.token.clone()),
        ..Configuration::default()
    };

    match &cli.command {
        Commands::Llm(sc) => match &sc.subcommand {
            Llmcommands::List => {
                let blah = get_openai_models_v1_models_get(&conf, None).await?;

                for item in blah.as_object().unwrap() {
                    let (key, val) = item;
                    if key == "data" {
                        for v in val.as_array().unwrap() {
                            for i in v.as_object().unwrap() {
                                let (k, v) = i;
                                if k == "name" {
                                    if let Some(j) = v.as_str() {
                                        println!("{}", j);
                                    }
                                }
                            }
                        }
                    };
                }
            }
        },
    };

    Ok(())
}
