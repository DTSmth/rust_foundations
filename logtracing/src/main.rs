#[tracing::instrument]
async fn hello_world() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up");
    tracing::warn!("are you sure?");
    tracing::error!("utoh thats not supposed to happen");
    hello_world().await;

    Ok(())
}
