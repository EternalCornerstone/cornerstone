use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{HttpServer, middleware, App};
use actix_files;

use crate::api::APIRoutes;

pub async fn start_http_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(actix_files::Files::new("/tmp", "./tmp").show_files_listing())
            .service(APIRoutes::get_resource(APIRoutes::Index))
            .service(APIRoutes::get_resource(APIRoutes::About))
            .service(APIRoutes::get_resource(APIRoutes::Contact))
            .service(APIRoutes::get_resource(APIRoutes::Profile))
            .service(APIRoutes::get_resource(APIRoutes::Files))
            .service(APIRoutes::get_resource(APIRoutes::Products))
            .service(APIRoutes::get_resource(APIRoutes::Login))
            .service(APIRoutes::get_resource(APIRoutes::Register))
            .service(APIRoutes::get_resource(APIRoutes::CMS))
            .service(APIRoutes::get_resource(APIRoutes::ForgotPassword))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
