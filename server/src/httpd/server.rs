use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use paperclip::actix::{self, OpenApiExt, HttpResponseWrapper};
use log::info;

use super::talents::{
    get_talents, create_talent, get_talent, update_talent, delete_talent,
};
use super::jobs::{
    get_jobs, create_job, get_job, update_job, delete_job,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::SqlitePool,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db_pool = crate::database::init_pool(database_url).await?;
        Ok(Self { db_pool })
    }
}

#[actix::get("/")]
pub async fn swagger_ui() -> HttpResponseWrapper {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Talent Pool API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui.css" />
    <style>
        html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
        *, *:before, *:after { box-sizing: inherit; }
        body { margin:0; background: #fafafa; }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: '/api/v1',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
                plugins: [SwaggerUIBundle.plugins.DownloadUrl],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>"#;
    HttpResponseWrapper(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
    )
}

pub async fn run_server(rest_host: &str, rest_port: u16, database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new(database_url).await?;

    let bind_address = format!("{}:{}", rest_host, rest_port);
    info!("Starting X Talent Pool Server on http://{}", bind_address);

    let server = HttpServer::new(move || {
        // CORS configuration for development
        // Note: allow_any_origin() cannot be used with supports_credentials()
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap_api()
            .with_json_spec_at("/api/v1")
            .service(swagger_ui)
            // Talent routes
            .service(get_talents)
            .service(create_talent)
            .service(get_talent)
            .service(update_talent)
            .service(delete_talent)
            // Job routes
            .service(get_jobs)
            .service(create_job)
            .service(get_job)
            .service(update_job)
            .service(delete_job)
            .build()
    })
    .bind(&bind_address)?
    .run();

    server.await?;

    Ok(())
}