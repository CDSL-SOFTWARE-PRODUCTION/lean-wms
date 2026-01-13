mod api;
mod app_state;
mod config;
mod database;
mod middleware;
mod services;

use axum::http::{HeaderValue, Method};
use axum::Router;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
  // Initialize logger
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  // Load configuration
  let config = config::Config::from_env();

  // Initialize database connection
  log::info!("Connecting to database...");
  let db = match database::connection::connect(&config.database_url).await {
    Ok(conn) => {
      log::info!("Database connection established");

      // Run migrations
      log::info!("Running migrations...");
      match crate::database::migration::Migrator::up(&conn, None).await {
        Ok(_) => log::info!("Migrations completed successfully"),
        Err(e) => log::error!("Migration failed: {}", e),
      }

      conn
    }
    Err(e) => {
      log::error!("Failed to connect to database: {}", e);
      log::error!("Please ensure PostgreSQL is running and DATABASE_URL is correct");
      std::process::exit(1);
    }
  };

  // Create app state
  let app_state = app_state::AppState {
    db: db.clone(), // Clone db connection (it's cheap, just a handle)
    config: config.clone(),
  };

  // Initialize RSPC router
  let rspc_router = api::router::mount();

  // Build application router with API routes
  let app = Router::new()
    .nest("/api/auth", api::auth::router())
    .nest("/api/products", api::products::router())
    .nest("/api/orders", api::orders::router())
    .nest("/api/sync", api::sync::router())
    // RSPC integration for Axum 0.7
    .nest(
      "/rspc",
      rspc_axum::endpoint(rspc_router.clone(), {
        let state = app_state.clone();
        move || api::router::Ctx { state: state.clone() }
      }),
    )
    .layer(
      CorsLayer::new()
        .allow_origin(AllowOrigin::list(
          config
            .cors_origins
            .iter()
            .map(|origin| HeaderValue::from_str(origin).unwrap())
            .collect::<Vec<_>>(),
        ))
        .allow_methods([
          Method::GET,
          Method::POST,
          Method::PUT,
          Method::DELETE,
          Method::PATCH,
          Method::OPTIONS,
        ])
        .allow_headers([
          axum::http::header::CONTENT_TYPE,
          axum::http::header::AUTHORIZATION,
          axum::http::header::ACCEPT,
        ])
        .allow_credentials(true),
    )
    .with_state(app_state);

  // Run server (Axum 0.7 style)
  let addr = SocketAddr::from(([0, 0, 0, 0], config.api_port));
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  log::info!("🚀 Server listening on http://{}", addr);
  log::info!("📡 API endpoints available at:");
  log::info!("   - http://localhost:{}/api/auth", config.api_port);
  log::info!("   - http://localhost:{}/rspc", config.api_port);

  axum::serve(listener, app).await.unwrap();
}
