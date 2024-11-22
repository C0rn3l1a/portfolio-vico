use std::env;
use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;
use lazy_static::lazy_static;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, EnvFilter, Layer};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logger();

    let app = Router::new()
        .route("/", get(home))
        .route("/work", get(under_construction))
        .route("/about", get(about))
        .route("/design_system", get(design_system))
        .route("/healthz", get(health))
        .nest_service("/static", ServeDir::new("static"));

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let context = Context::new();
    let s = match TEMPLATES.render("pages/home.html", &context) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {:?}", e);
            String::from("Error")
        }
    };
    return Html(s);
}

async fn under_construction() -> Html<String> {
    let context = Context::new();
    let s = match TEMPLATES.render("pages/under_construction.html", &context) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {:?}", e);
            String::from("Error")
        }
    };
    return Html(s);
}

async fn about() -> Html<String> {
    let context = Context::new();
    let s = match TEMPLATES.render("pages/about.html", &context) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {:?}", e);
            String::from("Error")
        }
    };
    return Html(s);
}

async fn design_system() -> Html<String> {
    let context = Context::new();
    let s = match TEMPLATES.render("pages/design_system.html", &context) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {:?}", e);
            String::from("Error")
        }
    };
    return Html(s);
}

async fn health() -> String {
    return String::from("Server Alive");
}

fn setup_logger() {
    let logger = tracing_subscriber::fmt::layer().with_filter(filter::LevelFilter::DEBUG);
    let log_level = env::var("LOG_LEVEL").unwrap_or(String::from("None"));
    let port = env::var("PORT").unwrap_or(String::from("3000"));

    tracing_subscriber::registry()
        .with(logger)
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();

    tracing::info!("Process started with:");
    tracing::info!("---");
    tracing::info!("- LOG_LEVEL: {log_level}");
    tracing::info!("- Server is listening: http://localhost:{port}");
}
