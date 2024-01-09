pub mod generics;
pub mod components;
pub mod helpers;
pub mod pages;
pub mod form;

// The UI library will recieve a specific page to go to. then within that page each page will be defined by a specific structure or potential of structures, then it
// is provided a set of parameters in order to build that specific page. This ui library should be independent of any server side functionality and only take in some
// requirements and return a built version of the page that has been requested for. shouldn't have to await. UI cannot send data out only return data that is requested.
// If there is any data required for the page then it must be sent to the function so that it can be built. 

use actix_web::HttpResponse;

use crate::{ui::{generics::template::return_template, pages::{index::index_page, login::login_page, register::register_page, about::about_page, contact::contact_page, profile::profile_page, products::products_page, content_management::content_management_page, files::files_page, forgotten_password::forgotten_password_page}}, api::APIRoutes};

pub async fn build_set_page(page: APIRoutes) -> HttpResponse {
    let build = match page {
        APIRoutes::Files => files_page(),
        APIRoutes::Index => index_page(),
        APIRoutes::Login => login_page(),
        APIRoutes::Register => register_page(),
        APIRoutes::About => about_page(),
        APIRoutes::Contact => contact_page(),
        APIRoutes::Profile => profile_page(),
        APIRoutes::Products => products_page(),
        APIRoutes::CMS => content_management_page(),
        APIRoutes::ForgotPassword => forgotten_password_page(),
    };
    let completed_build = return_template(build);
    HttpResponse::Ok().body(completed_build)
}

