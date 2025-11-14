use clap::Parser;
use anyhow::Result;
use tracing::{info};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(name = "dev", about = "A small dev utility")]
struct Args {

    #[arg(short, long)]
    verbose: bool,


    #[arg(short, long)]
    input: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(if args.verbose { "debug" } else { "info" })
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting dev tool");
    if let Some(inp) = args.input {
        info!("Received input: {}", inp);
        do_work(&inp).await?;
    } else {
        info!("No input provided, running default flow");
        do_default_work().await?;
    }

    Ok(())
}

async fn do_work(input: &str) -> Result<()> {
    info!("Doing async work on: {}", input);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    info!("Done");
    Ok(())
}

async fn do_default_work() -> Result<()> {
    info!("Running default task");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok(())
}
