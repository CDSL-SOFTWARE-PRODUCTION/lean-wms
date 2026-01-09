// Products endpoints

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::database::entities::product;
use sea_orm::*;

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub master_sku: String,
    pub unit: Option<String>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub master_sku: String,
    pub unit: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_products).post(create_product))
        .route("/:id", get(get_product).put(update_product).delete(delete_product))
}

async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<product::Model>>, (StatusCode, Json<ErrorResponse>)> {
    let products = product::Entity::find()
        .all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;
    
    Ok(Json(products))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<product::Model>, (StatusCode, Json<ErrorResponse>)> {
    let product = product::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    match product {
        Some(p) => Ok(Json(p)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Product not found".to_string(),
            }),
        )),
    }
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<product::Model>, (StatusCode, Json<ErrorResponse>)> {
    let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
    
    let new_product = product::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        master_sku: Set(payload.master_sku),
        unit: Set(payload.unit),
        status: Set("ACTIVE".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = new_product
        .insert(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    Ok(Json(result))
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<product::Model>, (StatusCode, Json<ErrorResponse>)> {
    let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
    
    let product = product::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    match product {
        Some(p) => {
            let mut am: product::ActiveModel = p.into();
            am.name = Set(payload.name);
            am.master_sku = Set(payload.master_sku);
            am.unit = Set(payload.unit);
            am.updated_at = Set(now);

            let result = am.update(&state.db).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Database error: {}", e),
                    }),
                )
            })?;

            Ok(Json(result))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Product not found".to_string(),
            }),
        )),
    }
}

async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let result = product::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    if result.rows_affected == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Product not found".to_string(),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

