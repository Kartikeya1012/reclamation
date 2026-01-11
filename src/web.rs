use axum::{
    extract::Path,
    http::{HeaderValue, Method, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tower_http::cors::{CorsLayer, Any};
use crate::{triage_folder, clean_folder, list_manifests, restore_manifest};
use crate::classify::Classification;
use crate::classify::reason;

#[derive(Serialize)]
pub struct TriageResult {
    auto_safe: Vec<FileItem>,
    needs_review: Vec<FileItem>,
    do_not_touch: Vec<FileItem>,
}

#[derive(Serialize)]
pub struct FileItem {
    path: String,
    reason: Option<String>,
}

#[derive(Deserialize)]
pub struct CleanRequest {
    path: String,
}

#[derive(Serialize)]
pub struct CleanResponse {
    success: bool,
    message: String,
    manifest_id: Option<String>,
}

#[derive(Serialize)]
pub struct ListResponse {
    manifests: Vec<String>,
}

async fn triage_handler(Path(path): Path<String>) -> Result<Json<TriageResult>, StatusCode> {
    let path_buf = PathBuf::from(path);
    let (auto, review, skip) = triage_folder(&path_buf)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let auto_safe: Vec<FileItem> = auto.iter()
        .map(|p| FileItem {
            path: p.display().to_string(),
            reason: reason(p, Classification::AutoSafe),
        })
        .collect();
    
    let needs_review: Vec<FileItem> = review.iter()
        .map(|p| FileItem {
            path: p.display().to_string(),
            reason: None,
        })
        .collect();
    
    let do_not_touch: Vec<FileItem> = skip.iter()
        .map(|p| FileItem {
            path: p.display().to_string(),
            reason: None,
        })
        .collect();
    
    Ok(Json(TriageResult {
        auto_safe,
        needs_review,
        do_not_touch,
    }))
}

async fn clean_handler(Json(req): Json<CleanRequest>) -> Result<Json<CleanResponse>, StatusCode> {
    let path_buf = PathBuf::from(req.path);
    match clean_folder(&path_buf) {
        Ok(manifest) => Ok(Json(CleanResponse {
            success: true,
            message: format!("Quarantined {} items", manifest.entries.len()),
            manifest_id: Some(manifest.id),
        })),
        Err(e) => Ok(Json(CleanResponse {
            success: false,
            message: e.to_string(),
            manifest_id: None,
        })),
    }
}

async fn list_handler() -> Result<Json<ListResponse>, StatusCode> {
    let manifests = list_manifests()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ListResponse { manifests }))
}

async fn restore_handler(Path(id): Path<String>) -> Result<Json<CleanResponse>, StatusCode> {
    match restore_manifest(&id) {
        Ok(_) => Ok(Json(CleanResponse {
            success: true,
            message: format!("Restored manifest: {}", id),
            manifest_id: Some(id),
        })),
        Err(e) => Ok(Json(CleanResponse {
            success: false,
            message: e.to_string(),
            manifest_id: None,
        })),
    }
}

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .route("/api/triage/:path", get(triage_handler))
        .route("/api/clean", post(clean_handler))
        .route("/api/list", get(list_handler))
        .route("/api/restore/:id", post(restore_handler))
        .layer(cors)
}
