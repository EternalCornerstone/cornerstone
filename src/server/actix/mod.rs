use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{HttpServer, middleware, App, web, HttpResponse, Resource, Responder};
use actix_files;

use crate::{api::{APIRoute, Api}, ui::{build_set_page, Pages}};

pub mod api;
    
pub struct ActixServer {
    routes: Vec<APIRoute>,
}

impl ActixServer {
    pub async fn new(address: &str, routes: Vec<APIRoute>) -> std::io::Result<()> {
        //FIND a way to implement the dynamic routing based on the routes param.
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .app_data(web::Data::new(TempFileConfig::default().directory("./tmp")))
                .service(actix_files::Files::new("/tmp", "./tmp").show_files_listing())
                .service(Api::get_resource(Api::IndexPage))
                .service(Api::get_resource(Api::AboutPage))
                .service(Api::get_resource(Api::ContactPage))
                .service(Api::get_resource(Api::ProfilePage))
                .service(Api::get_resource(Api::FilesPage))
                .service(Api::get_resource(Api::ProductsPage))
                .service(Api::get_resource(Api::LoginPage))
                .service(Api::get_resource(Api::RegisterPage))
                .service(Api::get_resource(Api::CMSPage))
                .service(Api::get_resource(Api::ForgotPasswordPage))
    
        })
        .bind(address)?
        .workers(2)
        .run()
        .await
    }    
  

    // Add a route to the server
    pub fn add_route(&mut self, api: APIRoute) {
        self.routes.push(api);
    }
    pub async fn get_resource(api: Api) -> Resource {
        match api {
            Api::IndexPage => {
                web::resource(Api::IndexPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Index).await }))
            },
            Api::AboutPage => {
                web::resource(Api::AboutPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::About).await }))
            },
            Api::ProductsPage => {
                web::resource(Api::ProductsPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Products).await }))
            },
            Api::ContactPage => {
                web::resource(Api::ContactPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Contact).await }))

            },
            Api::RegisterPage => {
                web::resource(Api::RegisterPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Register).await }))
            },
            Api::LoginPage => {
                web::resource(Api::LoginPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Login).await }))
            },
            Api::ProfilePage => {
                web::resource(Api::ProfilePage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Profile).await }))
            },
            Api::FilesPage => {
                web::resource(Api::FilesPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::Files).await }))
            },
            Api::CMSPage => {
                web::resource(Api::CMSPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::ContentManagement).await }))
            },
            Api::ForgotPasswordPage => {
                web::resource(Api::ForgotPasswordPage.url())
                .route(web::get().to(|| async { Self::http_response(Pages::ForgottenPassword).await }))
            },
        }
    }
    pub async fn http_response(page: Pages) -> HttpResponse {
        let page = build_set_page(page).await;
        HttpResponse::Ok().body(format!("Handler: {}", page))
    
    }
    // pub async fn handle_route(api: Api) -> impl Responder {
    //     let handler = match api {
    //         Api::IndexPage => Self::http_response(Pages::Index),
    //         Api::AboutPage => Self::http_response(Pages::About),
    //         Api::ProductsPage => Self::http_response(Pages::Products),
    //         Api::ContactPage => Self::http_response(Pages::Contact),
    //         Api::RegisterPage => Self::http_response(Pages::Register),
    //         Api::LoginPage => Self::http_response(Pages::Login),
    //         Api::ProfilePage => Self::http_response(Pages::Profile),
    //         Api::FilesPage => Self::http_response(Pages::Files),
    //         Api::CMSPage => Self::http_response(Pages::ContentManagement),
    //         Api::ForgotPasswordPage => Self::http_response(Pages::ForgottenPassword),
    //     }.await;
    //     // Implement how the handler string is translated into a response
    //     handler
    // }
    
    
}
