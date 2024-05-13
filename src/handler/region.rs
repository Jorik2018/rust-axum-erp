use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    model::{RegionModel},
    schema::{FilterOptions},
    AppState,
};

pub async fn region_list_handler(
    Path((from, to)): Path<(i32, i32)>,
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let Query(opts) = opts.unwrap_or_default();
    
    let offset = from;// = (opts.page.unwrap_or(1) - 1) * limit;
    // Query with macro
    let notes = sqlx::query_as!(
        RegionModel,
        r#"SELECT codigo_dpto as code, nombre_dpto as name FROM drt_departamento LIMIT ? OFFSET ?"#,
        to,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

        // Query to count total regions
        let total_count: i64 = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM drt_departamento"#
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let note_responses = notes
        .iter()
        //.map(|note| to_note_response(&note))
        .map(|region| RegionModel {
            code: region.code.clone(),
            name: region.name.clone(),
        })
        .collect::<Vec<RegionModel>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": total_count,
        "data": note_responses
    });

    Ok(Json(json_response))
}
