use axum::{routing::get, Router};
use clap::Parser;
use log::{debug, info};
use std::env;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, default_value_t = 5000)]
    port: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.debug {
        env::set_var("RUST_LOG", "info,ghost_chat=debug");
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    debug!("Constructing router");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let address = format!("{}:{}", args.host, args.port);
    info!("Running on {address}");
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
