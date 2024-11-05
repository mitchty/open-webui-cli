// All the logic is in cli not here, the princess is in another castle mario.

#[tokio::main]
async fn main() {
    open_webui_cli::cli::run().await.expect("ok")
}
