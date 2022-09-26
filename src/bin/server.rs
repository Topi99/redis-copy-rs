use redis_copy_rs::{server, DEFAULT_PORT, Result};
use structopt::StructOpt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::from_args();
  let port = cli.port.as_deref().unwrap_or(DEFAULT_PORT);

  let listener = TcpListener::bind(
    format!("localhost:{port}")
  ).await.unwrap();

  server::run(listener).await;

  Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name="paciencia-server", version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"), about="A Redis server")]
struct Cli {
    #[structopt(name="port", long = "--port")]
    port: Option<String>,
}
