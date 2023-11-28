use actix_web::{web, App, HttpServer, Responder, HttpResponse, web::Data};
use std::fs;

// Function to read models from the directory
fn read_models() -> Vec<String> {
    if let Ok(entries) = fs::read_dir("./models") {
        entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.path().file_name().map(|s| s.to_string_lossy().into_owned()))
            .collect()
    } else {
        eprintln!("Warning: 'models' directory not found. No models will be loaded.");
        Vec::new()
    }
}


async fn get_models(data: web::Data<Vec<String>>) -> impl Responder {
    HttpResponse::Ok().json(data.get_ref())
}

async fn download_binary() -> impl Responder {
    // Replace this with logic to download and serve the binary
    "Binary download initiated"
}

async fn run_aws_spot_instance() -> impl Responder {
    // Replace this with logic to spin up an AWS spot instance
    // For now, it's just a placeholder response
    "AWS Spot Instance spin-up initiated"
}

async fn home() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("home.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let models = Data::new(read_models()); // Wrap models in Data

    HttpServer::new(move || {
        App::new()
            .app_data(models.clone()) // Use .app_data() instead of .data()
            .route("/", web::get().to(home))
            .route("/models", web::get().to(get_models))
            .route("/download", web::get().to(download_binary))
            .route("/run", web::get().to(run_aws_spot_instance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
