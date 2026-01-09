// Database connection setup

pub async fn connect(database_url: &str) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    sea_orm::Database::connect(database_url).await
}

