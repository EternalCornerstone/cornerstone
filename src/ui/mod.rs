pub mod generics;
pub mod components;
pub mod helpers;
pub mod pages;
pub mod form;

// The UI library will recieve a specific page to go to. then within that page each page will be defined by a specific structure or potential of structures, then it
// is provided a set of parameters in order to build that specific page. This ui library should be independent of any server side functionality and only take in some
// requirements and return a built version of the page that has been requested for. shouldn't have to await. UI cannot send data out only return data that is requested.
// If there is any data required for the page then it must be sent to the function so that it can be built. 


use crate::ui::{generics::template::return_template, pages::{index::index_page, login::login_page, register::register_page, about::about_page, contact::contact_page, profile::profile_page, products::products_page, content_management::content_management_page, files::files_page, forgotten_password::forgotten_password_page}};

pub async fn build_set_page(page: Pages) -> String {
    let build = match page {
        Pages::Files => files_page(),
        Pages::Index => index_page(),
        Pages::Login => login_page(),
        Pages::Register => register_page(),
        Pages::About => about_page(),
        Pages::Contact => contact_page(),
        Pages::Profile => profile_page(),
        Pages::Products => products_page(),
        Pages::ContentManagement => content_management_page(),
        Pages::ForgottenPassword => forgotten_password_page(),
    };
    let completed_build = return_template(build);
    completed_build
}

pub enum Pages {
    Index,
    About,
    Contact,
    ContentManagement,
    Files,
    ForgottenPassword,
    Login,
    Products,
    Profile,
    Register
}