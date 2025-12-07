use clap::Parser;
use log::info;
use talent_server::httpd::server::run_server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host to bind the server to
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port to bind the server to
    #[arg(long, default_value_t = 8080)]
    pub port: u16,

    /// Database URL
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite://talents.db?mode=rwc")]
    pub database_url: String,

    /// Grok service URL for resume analysis
    #[arg(long, env = "GROK_SERVICE_URL", default_value = "http://localhost:8001")]
    pub grok_service_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let args = Args::parse();
    info!("Starting server with host: {}, port: {}", args.host, args.port);
    info!("HTTP API hosted at: http://{}:{}/", args.host, args.port);
    info!("OpenAPI docs: http://{}:{}/", args.host, args.port);
    info!("Swagger UI: http://{}:{}/", args.host, args.port);

    run_server(&args.host, args.port, &args.database_url, &args.grok_service_url).await?;

    Ok(())
}