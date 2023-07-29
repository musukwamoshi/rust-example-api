use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use entity::*;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use service::*;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn add_user(
    req_body: web::Json<user::Model>,
    application_state: web::Data<AppState>,
) -> HttpResponse {
    let _ =
        user_mutation::UserMutation::create_user(&application_state.conn, req_body.into_inner())
            .await;
    HttpResponse::Ok().body("success")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //api set up
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // let host = env::var("HOST").expect("HOST is not set in .env file");
    // let port = env::var("PORT").expect("PORT is not set in .env file");
    // let server_url = format!("{host}:{port}");

    let conn = Database::connect(&db_url).await.unwrap();
    //Migrator::up(conn, None).await.unwrap();
    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/user/add", web::post().to(add_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
