use actix_web::{web, App, HttpServer, Responder, HttpResponse, web::Data};
use std::fs;
use serde_derive::Deserialize;
use std::process::Command;
//use std::fs::{self, Permissions};

#[derive(Deserialize)]
struct ExecuteModel {
    model: String,
    prompt: String,
}


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

/*execute model on submit

async fn run_model(query: web::Query<ModelQuery>) -> impl Responder {
    let args: Vec<&str> = query.prompt.split_whitespace().collect();
    if args.is_empty() {
        return HttpResponse::BadRequest().body("Prompt format is incorrect.");
    }
    
    let output = Command::new("./models/".to_owned() + args[0])
        .args(&args[1..])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout);
            HttpResponse::Ok().json(result.to_string())
        },
        Ok(output) => {
            let error = String::from_utf8_lossy(&output.stderr);
            HttpResponse::InternalServerError().body(error.to_string())
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}*/

// Handler for executing a model command
// Handler for executing a model command
async fn execute_command(form: web::Form<ExecuteModel>) -> impl Responder {
    let model_path = format!("./models/{}", form.model);
    // Give run permission on file
    let _resp = Command::new("chmod")
        .arg("-R")
        .arg("755")
        .arg("./models/")
        .spawn()
        .expect("Failed to give execution permissions");

    let output = Command::new(model_path)
        .arg("--prompt")
        .arg(&form.prompt)
        .output();

    println!{"{:?}",output};

    match output {
        Ok(output) if output.status.success() => {
            HttpResponse::Ok().content_type("text/plain").body(output.stdout)
        },
        Ok(output) => {
            HttpResponse::BadRequest().content_type("text/plain").body(output.stderr)
        },
        Err(e) => {
            HttpResponse::InternalServerError().content_type("text/plain").body(format!("{}", e))
        },
    }
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
            .route("/execute-command", web::post().to(execute_command))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
