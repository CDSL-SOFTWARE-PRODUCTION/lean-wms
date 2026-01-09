// SeaORM migration CLI entry point

use sea_orm_migration::prelude::*;
use lean_wms_api::database::migration::Migrator;

#[tokio::main]
async fn main() {
    cli::run_cli(Migrator).await;
}

