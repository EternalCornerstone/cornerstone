use crate::ui::{generics::template::return_template, pages::{index::index_page, about::about_page, products::products_page, contact::contact_page, login::login_page, profile::profile_page, register::register_page, files::files_page, content_management::content_management_page, forgotten_password::forgotten_password_page}};

#[derive(Debug, Clone)]
pub struct APIRoute {
    pub name: Api,
    pub url: &'static str,
    pub method: HTTPMethod,
    pub handler: String
}
#[derive(Debug, Clone, Copy)]
// change this to determine what will be included in the application.
pub enum Api {
    IndexPage,
    AboutPage,
    ProductsPage,
    ContactPage,
    RegisterPage,
    LoginPage,
    ProfilePage,
    FilesPage,
    CMSPage,
    ForgotPasswordPage,
}
impl Api {
    pub async fn new() -> Vec<APIRoute> {
        let routes = Api::route_vec();
        let configuration: Vec<APIRoute> = Api::configure_routes(routes).await;
        configuration
    }
    pub fn route_vec() -> Vec<Api> {
        vec![
            Api::IndexPage, 
            Api::AboutPage, 
            Api::ProductsPage, 
            Api::ContactPage, 
            Api::RegisterPage, 
            Api::LoginPage, 
            Api::ProfilePage, 
            Api::FilesPage, 
            Api::CMSPage, 
            Api::ForgotPasswordPage
        ]
    }    
    pub async fn configure_routes(vec: Vec<Api>) -> Vec<APIRoute>{
        let mut routes = vec![];
        for route in vec {
            routes.push(APIRoute {
                name: route,
                url: route.url(),
                method: route.method(),
                handler: route.handler().await
            });
        }
        routes
    }
    
    pub fn url(&self) -> &'static str {
        match self {
            Api::IndexPage => "/",
            Api::AboutPage => "/about",
            Api::ProductsPage => "/products",
            Api::ContactPage => "/contact",
            Api::RegisterPage => "/register",
            Api::LoginPage => "/login",
            Api::ProfilePage => "/profile",
            Api::FilesPage => "/files",
            Api::CMSPage => "/cms",
            Api::ForgotPasswordPage => "/forgot_password",
        }
    }

    pub fn method(&self) -> HTTPMethod {
        match self {
            Api::IndexPage => HTTPMethod::GET,
            Api::AboutPage => HTTPMethod::GET,
            Api::ProductsPage => HTTPMethod::GET,
            Api::ContactPage => HTTPMethod::GET,
            Api::RegisterPage => HTTPMethod::GET,
            Api::LoginPage => HTTPMethod::GET,
            Api::ProfilePage => HTTPMethod::GET,
            Api::FilesPage => HTTPMethod::GET,
            Api::CMSPage => HTTPMethod::GET,
            Api::ForgotPasswordPage => HTTPMethod::GET,
        }
    }

    pub async fn handler(&self) -> String {
        // Return the appropriate handler function.
        let build = match self {
            Api::IndexPage => index_page(),
            Api::AboutPage => about_page(),
            Api::ProductsPage => products_page(),
            Api::ContactPage => contact_page(),
            Api::RegisterPage => register_page(),
            Api::LoginPage => login_page(),
            Api::ProfilePage => profile_page(),
            Api::FilesPage => files_page(),
            Api::CMSPage => content_management_page(),
            Api::ForgotPasswordPage => forgotten_password_page(),
        };
        let completed_build = return_template(build);
        completed_build
    }

}
// extend this as required.
#[derive(Debug, Clone)]

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

// call some service from here which then gathers the data it needs and puts together the ui with that data which is then returned here and served as a http response.