use actix_web::{web, App, HttpServer, Responder, HttpResponse};

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
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(home))
        .route("/download", web::get().to(download_binary))
        .route("/run", web::get().to(run_aws_spot_instance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
