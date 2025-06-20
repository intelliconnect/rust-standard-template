use axum::{ extract::Extension, routing::{get, post}, Router};
use tower_http::cors::CorsLayer;
use sqlx::postgres::PgPoolOptions;
use once_cell::sync::Lazy;
use dotenv::dotenv;
mod model;
mod controller;
mod routes;
mod middleware;
mod utils;
mod error;

// Static variables
static KEYS: Lazy<model::auth_models::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("set JWT_SECRET env variable");
    model::auth_models::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main(){
    // sentry initialization 
    // let _guard = if dev_env == "prod" {
    //     Some(sentry::init(("https://9f5857017761b7d034501debc36ecaed@o4504557842071552.ingest.sentry.io/4506534054920192", sentry::ClientOptions {
    //         release: sentry::release_name!(),
    //         ..Default::default()
    //     })))
    // } else {
    //     None
    // };

    let cors_layer = CorsLayer::permissive();
    
    dotenv().ok();
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await;

    let pool = pool.expect("could not connect to database");        

    let app = Router::new()
        .route("/", get(|| async{"Success"}))
        .route("/login", post(controller::login::get_authenticate))
        .nest("/api", routes::routes())
        .layer(cors_layer)
        .layer(Extension(pool));
    
    let addr: std::net::SocketAddr= std::net::SocketAddr::from(([0,0,0,0],5000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server")
}
