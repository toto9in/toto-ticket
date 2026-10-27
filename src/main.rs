mod domain;
mod http;
mod state;

use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use state::AppState;

const DEFAULT_PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads variables from `.env` when the file exists; real env vars take precedence.
    dotenvy::dotenv().ok();

    let port = port_from_args()?;

    let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set")?;
    let db = PgPoolOptions::new().connect(&database_url).await?;
    sqlx::migrate!().run(&db).await?;

    let state = AppState::new(db);
    let app = http::router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

/// Reads `--port <number>` from the command line, falling back to `DEFAULT_PORT`.
fn port_from_args() -> Result<u16, String> {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--port" {
            let value = args.next().ok_or("missing value for --port")?;
            return value.parse().map_err(|_| format!("invalid port: {value}"));
        }
    }

    Ok(DEFAULT_PORT)
}
