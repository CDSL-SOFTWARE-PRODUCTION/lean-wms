// Authentication endpoints

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{post, Router},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::database::entities::user;
use crate::services::auth::AuthService;
use sea_orm::*;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let auth_service = AuthService::new(&state.config);
    
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(payload.username))
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

    let user = user.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: "Invalid username or password".to_string(),
        }),
    ))?;

    let is_valid = AuthService::verify_password(&payload.password, &user.password_hash)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Error verifying password".to_string(),
                }),
            )
        })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid username or password".to_string(),
            }),
        ));
    }

    let user_id = user.id.to_string();
    let token = auth_service.generate_token(&user_id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to generate token".to_string(),
            }),
        )
    })?;
    
    let refresh_token = auth_service.generate_refresh_token(&user_id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to generate refresh token".to_string(),
            }),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            role: user.role,
        },
    }))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let auth_service = AuthService::new(&state.config);
    
    let password_hash = AuthService::hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to hash password".to_string(),
            }),
        )
    })?;

    let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
    let user_id = Uuid::new_v4();

    let new_user = user::ActiveModel {
        id: Set(user_id),
        username: Set(payload.username),
        password_hash: Set(password_hash),
        name: Set(payload.name),
        role: Set(payload.role.unwrap_or_else(|| "WORKER".to_string())),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let user = new_user.insert(&state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to create user: {}", e),
            }),
        )
    })?;

    let token = auth_service.generate_token(&user_id.to_string()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to generate token".to_string(),
            }),
        )
    })?;
    
    let refresh_token = auth_service.generate_refresh_token(&user_id.to_string()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to generate refresh token".to_string(),
            }),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            role: user.role,
        },
    }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let auth_service = AuthService::new(&state.config);
    
    let claims = auth_service.verify_refresh_token(&payload.refresh_token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid refresh token".to_string(),
            }),
        )
    })?;

    let user_uuid = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid user ID in token".to_string(),
            }),
        )
    })?;

    let user = user::Entity::find_by_id(user_uuid)
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

    let user = user.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: "User not found".to_string(),
        }),
    ))?;

    let token = auth_service
        .generate_token(&claims.sub)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to generate token".to_string(),
                }),
            )
        })?;
    
    let refresh_token = auth_service
        .generate_refresh_token(&claims.sub)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to generate refresh token".to_string(),
                }),
            )
        })?;

    Ok(Json(AuthResponse {
        token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            role: user.role,
        },
    }))
}
