use rspc::Router;
use std::sync::Arc;
use crate::app_state::AppState;
use sea_orm::ConnectionTrait;

pub struct Ctx {
    #[allow(dead_code)]
    pub state: AppState,
}

pub type RouterBuilder = rspc::RouterBuilder<Ctx>;

pub fn new() -> RouterBuilder {
    Router::new().config(rspc::Config::new().export_ts_bindings(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/core/src/bindings.ts"),
    ))
}

pub fn mount() -> Arc<Router<Ctx>> {
    let router = new()
        .query("version", |t| t(|_, _: ()| "0.1.0"))
        .query("health_check", |t| {
            t(|ctx, _: ()| async move {
                let db_status = match ctx.state.db.get_database_backend() {
                    sea_orm::DatabaseBackend::Postgres => "PostgreSQL - Connected",
                    _ => "Connected",
                };
                format!("Backend Status: OK, Database: {}", db_status)
            })
        })
        .build();
    
    router.arced()
}
