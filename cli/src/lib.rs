use std::fmt;

pub mod chat;
pub mod cli; // odd man out has cli::run()
pub mod delete;
pub mod link;
pub mod list;
pub mod new;

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
// fn rag_conf(uri: &str, port: &str, token: &str) -> rag::apis::configuration::Configuration {
//     let def_conf = rag::apis::configuration::Configuration::default();

//     rag::apis::configuration::Configuration {
//         base_path: format!("http://{}:{}{}", uri, port, def_conf.base_path),
//         bearer_access_token: Some(token.to_string()),
//         user_agent: Some("open-webui-cli/rust".to_owned()),
//         ..rag::apis::configuration::Configuration::default()
//     }
// }

pub fn webui_conf(uri: &str, port: &str, token: &str) -> webui::apis::configuration::Configuration {
    let def_conf = webui::apis::configuration::Configuration::default();

    webui::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, def_conf.base_path),
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..webui::apis::configuration::Configuration::default()
    }
}

pub fn default_conf(
    uri: &str,
    port: &str,
    token: &str,
) -> default::apis::configuration::Configuration {
    default::apis::configuration::Configuration {
        base_path: format!("http://{}:{}", uri, port), // base_path is http://localhost here... sigh consistency is for the birds apparently
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..default::apis::configuration::Configuration::default()
    }
}

pub fn ollama_conf(
    uri: &str,
    port: &str,
    token: &str,
) -> ollama::apis::configuration::Configuration {
    let def_conf = ollama::apis::configuration::Configuration::default();

    ollama::apis::configuration::Configuration {
        base_path: format!("http://{}:{}{}", uri, port, def_conf.base_path),
        bearer_access_token: Some(token.to_string()),
        user_agent: Some("open-webui-cli/rust".to_owned()),
        ..ollama::apis::configuration::Configuration::default()
    }
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
pub fn get_val(env_name: &str, cli_val: Option<String>, default: Option<String>) -> Option<String> {
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
