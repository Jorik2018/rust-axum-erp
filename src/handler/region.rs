use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{model::RegionModel, schema::FilterOptions, AppState};

pub async fn region_list_handler(
    Path((from, limit)): Path<(i32, i32)>,
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let opts = opts.unwrap_or_default().0;
    let offset = from; // = (opts.page.unwrap_or(1) - 1) * limit;
    let mut query = String::from(r#"SELECT codigo_dpto as code, nombre_dpto as name FROM drt_departamento"#);
    if limit > 0 {
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    }
    let notes: Vec<RegionModel> = sqlx::query_as::<_, RegionModel>(&query)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    if limit == 0 {
        let note_responses: Vec<RegionModel> = notes.into_iter().collect();
        return Ok(Json(serde_json::json!(note_responses)));
    }
    let total_count: i64 = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM drt_departamento"#)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let note_responses: Vec<RegionModel> = notes.into_iter().collect();

    // Construct the final response
    let json_response = serde_json::json!({
        "size": total_count,
        "data": note_responses
    });

    Ok(Json(json_response))

    /*if limit == 0 {
        let notes = sqlx::query_as!(
            RegionModel,
            query
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
        let note_responses = notes
        .iter()
        //.map(|note| to_note_response(&note))
        .map(|region| RegionModel {
            code: region.code.clone(),
            name: region.name.clone(),
        })
        .collect::<Vec<RegionModel>>();
    Ok(Json(serde_json::json!(note_responses)))
    } else {
        let notes = sqlx::query_as!(
            RegionModel,
            r#"SELECT codigo_dpto as code, nombre_dpto as name FROM drt_departamento LIMIT ? OFFSET ?"#,
            limit,
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
        let total_count: i64 = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM drt_departamento"#)
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
            "size": total_count,
            "data": note_responses
        });
        Ok(Json(json_response))
    }*/
}
