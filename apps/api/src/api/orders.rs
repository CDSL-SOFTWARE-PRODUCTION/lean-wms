// Orders endpoints

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
use crate::database::entities::order;
use sea_orm::*;

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub order_type: String,
    pub status: String,
    pub created_by: Uuid,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub order_number: String,
    pub order_type: String,
    pub created_by: Uuid,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_orders).post(create_order))
        .route("/:id", get(get_order).put(update_order).delete(delete_order))
}

async fn list_orders(
    State(state): State<AppState>,
) -> Result<Json<Vec<order::Model>>, (StatusCode, Json<ErrorResponse>)> {
    let orders = order::Entity::find()
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
    
    Ok(Json(orders))
}

async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<order::Model>, (StatusCode, Json<ErrorResponse>)> {
    let order = order::Entity::find_by_id(id)
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

    match order {
        Some(o) => Ok(Json(o)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Order not found".to_string(),
            }),
        )),
    }
}

async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<order::Model>, (StatusCode, Json<ErrorResponse>)> {
    let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
    
    let new_order = order::ActiveModel {
        id: Set(Uuid::new_v4()),
        order_number: Set(payload.order_number),
        order_type: Set(payload.order_type),
        status: Set("PENDING".to_string()),
        created_by: Set(payload.created_by),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = new_order
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

async fn update_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<order::Model>, (StatusCode, Json<ErrorResponse>)> {
    let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
    
    let order = order::Entity::find_by_id(id)
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

    match order {
        Some(o) => {
            let mut am: order::ActiveModel = o.into();
            am.order_number = Set(payload.order_number);
            am.order_type = Set(payload.order_type);
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
                error: "Order not found".to_string(),
            }),
        )),
    }
}

async fn delete_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let result = order::Entity::delete_by_id(id)
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
                error: "Order not found".to_string(),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

