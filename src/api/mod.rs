use actix_web::{App, HttpServer, HttpResponse, web, Resource, middleware};
use actix_files as fs;
use actix_multipart::form::tempfile::TempFileConfig;

use crate::{ui::build_set_page, file_upload::save_files};
pub enum APIRoutes {
    Index,
    About,
    Products,
    Contact,
    Register,
    Login,
    Profile,
    Files,
    CMS,
    ForgotPassword,
}

// call some service from here which then gathers the data it needs and puts together the ui with that data which is then returned here and served as a http response.
impl APIRoutes {
    pub async fn http_response(self) -> HttpResponse {

        // do some sort of logic for building the page outside of just getting the html.
        // match self {
            // fill in arms, build set page after getting some specific data from the services.
        // }
        build_set_page(self).await

    }
    pub fn get_resource(self) -> Resource {
        match self {
            Self::Index => {
                web::resource("/")
                .route(web::get().to(get_index_page))
            },
            Self::About => {
                web::resource("/about") 
                .route(web::get().to(get_about_page))
            },
            Self::Products => {
                web::resource("/products") 
                .route(web::get().to(get_products_page))
            },
            Self::Contact => {
                web::resource("/contact") 
                .route(web::get().to(get_contact_page))
                // .route(web::post().to(process_contact)) 

            },
            Self::Register => {
                web::resource("/register") 
                .route(web::get().to(get_registration_page))
                // .route(web::post().to(process_registration)) 
            },
            Self::Login => {
                web::resource("/login")
                .route(web::get().to(get_login_page))
                // .route(web::post().to(authenticate))
            },
            Self::Profile => {
                web::resource("/profile")
                .route(web::get().to(get_profile_page))
            },
            Self::Files => {
                web::resource("/file_upload")
                .route(web::get().to(get_files_page))
                .route(web::post().to(save_files))
            },
            Self::CMS => {
                web::resource("/cms")
                .route(web::get().to(get_cms_page))
            }
            Self::ForgotPassword => {
                web::resource("/forgotten_password")
                .route(web::get().to(get_forgotten_password_page))
                // .route(web::post().to(process_forgotten_password_reset_request))
            }
        }
    }
}

async fn get_login_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Login).await
}

async fn get_index_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Index).await
}

async fn get_products_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Products).await
}

async fn get_contact_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Contact).await
}

async fn get_about_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::About).await
}

async fn get_files_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Files).await
}

async fn get_profile_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Profile).await
}

async fn get_registration_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::Register).await
}
async fn get_cms_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::CMS).await

}
async fn get_forgotten_password_page() -> HttpResponse {
    APIRoutes::http_response(APIRoutes::ForgotPassword).await
}

pub async fn start_http_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(fs::Files::new("/tmp", "./tmp").show_files_listing())
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

