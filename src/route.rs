use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::note::{
        create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
        health_check_handler, note_list_handler,
    },
    handler::region::region_list_handler,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .nest(
            "/notes",
            Router::new()
                .route("/", post(create_note_handler))
                .route("/", get(note_list_handler))
                .route(
                    "/:id",
                    get(get_note_handler)
                        .patch(edit_note_handler)
                        .delete(delete_note_handler),
                ),
        )
        .nest(
            "/region",
            Router::new().route("/:from/:limit", get(region_list_handler)),
        )
        .with_state(app_state)
}
