use actix_web::{post, web, App, HttpServer, Responder};
use rumlibrs::config::Config;
use std::sync::{Arc, Mutex};

#[post("/config/save")]
async fn config_save(config: web::Data<Arc<Mutex<Config>>>) -> impl Responder {
    let mut config = config.lock().unwrap();
    if let Err(e) = config.save() {
        return web::Json(format!("Error saving config: {}", e));
    }
    web::Json("Config saved successfully".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Arc::new(Mutex::new(Config::new("rum".into())));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(config_save)
        // Add other services here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
