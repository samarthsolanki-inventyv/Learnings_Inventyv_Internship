use axum::{routing::get, Router};
use crate::api::*;
use crate::api::SharedState;

pub fn create_routes(state: SharedState) -> Router {
    Router::new()
        .route("/drivers", get(get_drivers).post(add_driver))
        .route(
            "/drivers/:id",
            get(get_driver)
                .put(update_driver)
                .delete(delete_driver),
        )
        .with_state(state)
}
