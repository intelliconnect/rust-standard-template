#[allow(unused_imports)]
use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::controller::handler::{example_create, example_delete, example_detail, example_list, example_update};

// This is the entry point to mount all routes
pub fn routes() -> Router {
    Router::new()
        // Add feature/module-specific routers here
        .nest("/example", example_routes())
        // .nest("/auth", auth_routes())
        // .nest("/users", user_routes())
}

// Template route group for a feature/module
fn example_routes() -> Router {
    Router::new()
        .route("/", get(example_list))              // GET /example
        .route("/", post(example_create))           // POST /example
        .route("/:id", get(example_detail))         // GET /example/:id
        .route("/:id", put(example_update))         // PUT /example/:id
        .route("/:id", delete(example_delete))      // DELETE /example/:id
}