use actix_web::{HttpResponse, Resource, web};

use crate::{api::Api, ui::{Pages, build_set_page}, file_upload::save_files};

impl Api {
    pub async fn http_response(page: Pages) -> HttpResponse {

        // do some sort of logic for building the page outside of just getting the html.
        // match self {
            // fill in arms, build set page after getting some specific data from the services.
        // }
        let response = build_set_page(page).await;
        HttpResponse::Ok().body(response)
    }

    pub fn get_resource(self) -> Resource {
        match self {
            Self::IndexPage => {
                web::resource(Api::IndexPage.url())
                .route(web::get().to(get_index_page))
            },
            Self::AboutPage => {
                web::resource(Api::AboutPage.url()) 
                .route(web::get().to(get_about_page))
            },
            Self::ProductsPage => {
                web::resource(Api::ProductsPage.url()) 
                .route(web::get().to(get_products_page))
            },
            Self::ContactPage => {
                web::resource(Api::ContactPage.url()) 
                .route(web::get().to(get_contact_page))
                // .route(web::post().to(process_contact)) 

            },
            Self::RegisterPage => {
                web::resource(Api::RegisterPage.url()) 
                .route(web::get().to(get_registration_page))
                // .route(web::post().to(process_registration)) 
            },
            Self::LoginPage => {
                web::resource(Api::LoginPage.url())
                .route(web::get().to(get_login_page))
                // .route(web::post().to(authenticate))
            },
            Self::ProfilePage => {
                web::resource(Api::ProfilePage.url())
                .route(web::get().to(get_profile_page))
            },
            Self::FilesPage => {
                web::resource(Api::FilesPage.url())
                .route(web::get().to(get_files_page))
                .route(web::post().to(save_files))
            },
            Self::CMSPage => {
                web::resource(Api::CMSPage.url())
                .route(web::get().to(get_cms_page))
            }
            Self::ForgotPasswordPage => {
                web::resource(Api::ForgotPasswordPage.url())
                .route(web::get().to(get_forgotten_password_page))
                // .route(web::post().to(process_forgotten_password_reset_request))
            }
        }
    }
}

async fn get_login_page() -> HttpResponse {
    Api::http_response(Pages::Login).await
}

async fn get_index_page() -> HttpResponse {
    Api::http_response(Pages::Index).await
}

async fn get_products_page() -> HttpResponse {
    Api::http_response(Pages::Products).await
}

async fn get_contact_page() -> HttpResponse {
    Api::http_response(Pages::Contact).await
}

async fn get_about_page() -> HttpResponse {
    Api::http_response(Pages::About).await
}

async fn get_files_page() -> HttpResponse {
    Api::http_response(Pages::Files).await
}

async fn get_profile_page() -> HttpResponse {
    Api::http_response(Pages::Profile).await
}

async fn get_registration_page() -> HttpResponse {
    Api::http_response(Pages::Register).await
}
async fn get_cms_page() -> HttpResponse {
    Api::http_response(Pages::ContentManagement).await

}
async fn get_forgotten_password_page() -> HttpResponse {
    Api::http_response(Pages::ForgottenPassword).await
}

