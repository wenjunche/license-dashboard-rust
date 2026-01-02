use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;

use crate::{models::*, db};

// List app configs with filters
pub async fn list_app_configs(
    State(pool): State<PgPool>,
    Query(query): Query<AppConfigQuery>,
) -> Result<Json<AppConfigListResponse>, AppError> {
    let response = db::list_app_configs(&pool, &query).await?;
    Ok(Json(response))
}

// Create new app config
pub async fn create_app_config(
    State(pool): State<PgPool>,
    Json(input): Json<CreateAppConfig>,
) -> Result<(StatusCode, Json<AppConfig>), AppError> {
    let config = db::create_app_config(&pool, input).await?;
    Ok((StatusCode::CREATED, Json(config)))
}

// Get app config by ID
pub async fn get_app_config(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<AppConfig>, AppError> {
    let config = db::get_app_config_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(config))
}

// Update app config
pub async fn update_app_config(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateAppConfig>,
) -> Result<Json<AppConfig>, AppError> {
    let config = db::update_app_config(&pool, id, input)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(config))
}

// Delete app config
pub async fn delete_app_config(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let deleted = db::delete_app_config(&pool, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

// Bulk update app configs
pub async fn bulk_update_app_configs(
    State(pool): State<PgPool>,
    Json(input): Json<BulkUpdateRequest>,
) -> Result<Json<Vec<AppConfig>>, AppError> {
    let configs = db::bulk_update_app_configs(&pool, &input.ids, &input.update).await?;
    Ok(Json(configs))
}

// Export app configs as CSV
pub async fn export_app_configs(
    State(pool): State<PgPool>,
    Query(query): Query<AppConfigQuery>,
) -> Result<Response, AppError> {
    let response = db::list_app_configs(&pool, &query).await?;
    
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["ID", "Contract", "URL", "UUID", "Name", "Billable", "Ignore Files"])?;
    
    for config in response.data {
        wtr.write_record(&[
            config.id.to_string(),
            config.contract,
            config.url,
            config.uuid,
            config.name,
            config.billable.map(|b| b.to_string()).unwrap_or_else(|| "NULL".to_string()),
            config.ignore_files.to_string(),
        ])?;
    }
    
    let csv_data = wtr.into_inner()?;
    
    Ok((
        StatusCode::OK,
        [("Content-Type", "text/csv"), ("Content-Disposition", "attachment; filename=app_configs.csv")],
        csv_data,
    ).into_response())
}

// Placeholder OAuth handlers (implement with openidconnect)
pub async fn google_login() -> impl IntoResponse {
    // TODO: Implement OAuth flow
    "Redirect to Google OAuth"
}

pub async fn google_callback() -> impl IntoResponse {
    // TODO: Handle OAuth callback and issue JWT
    "OAuth callback handler"
}

// Error handling
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    Csv(csv::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::Csv(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Csv(err) => {
                tracing::error!("CSV error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate CSV")
            }
        };
        
        (status, message).into_response()
    }
}
