mod client;
mod run;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = run::run().await {
        eprintln!("error occured while trying to run: {e}");
    }
    Ok(())
}
