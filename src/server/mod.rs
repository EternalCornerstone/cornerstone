use crate::{error::AppError, api::{Api, APIRoute}};
pub mod cornerstone;
pub mod actix;
#[derive(Debug)]
pub enum Server {
    Actix,
    Cornerstone,
    Nothing
}
pub trait RouteConfigurator {
    fn new() -> Self;
    fn add_route(&mut self, api: &Api);
    fn run(self);    
}

impl Server {
    pub async fn start_server(server: Server) -> Result<(), AppError>  {
        println!("starting server: {} {} {:?}", server.as_str(), server.as_command(), server);
        let api: Vec<APIRoute> = Api::new().await;
        match server {
            Server::Actix => {
                actix::ActixServer::new("127.0.0.1:8080", api).await.map_err(|e| e.into())
            },
            Server::Cornerstone => {
                println!("Starting the Cornerstone system :)");
                let mut corner: cornerstone::CornerstoneServer = cornerstone::CornerstoneServer::new("127.0.0.1:8080").unwrap();
                for route in api {
                    corner.add_route(route);
                }
                corner.run().await.map_err(|e| e.into())
            },
            Server::Nothing => {
                println!("Implement your own check out the code to see how to interface it into your own server :).");
                Ok(())
            },
        }
    }

    pub fn get_config(input: &str) -> &'static str {
        let trimmed = input.trim();
        match trimmed {
            "a" => Server::Actix.as_str(),
            "c" => Server::Cornerstone.as_str(),
            _ => Server::Nothing.as_str()
        }
    }    
    pub fn as_str(&self) -> &'static str {
        match self {
            Server::Actix => "Actix",
            Server::Cornerstone => "Cornerstone",
            Server::Nothing => "",
        }
    }
    pub fn as_command(&self) -> &'static str {
        match self {
            Server::Actix => "a", 
            Server::Cornerstone => "c",
            Server::Nothing => ""
        }
    }
    pub fn as_server(str: &str) -> Server {
        match str {
            "Actix" => Server::Actix,
            "Cornerstone" => Server::Cornerstone,
            _ => Server::Nothing
        }
    }
    pub fn config_variants_as_vec() -> Vec<Server> {
        vec![Server::Actix, Server::Cornerstone, Server::Nothing]
    }

    pub fn give_users_server_options() {
        let variants = Server::config_variants_as_vec();
        let mut options = String::from("Choose a server: ");
        for (i, variant) in variants.iter().enumerate() {
            if variant.as_str() != "" {
                options += &format!("{} [{}]{}", variant.as_str(), variant.as_command(), if i < variants.len() - 2 { ", " } else { "" });
            }
        }
        println!("{}", options);
    }
    // Existing methods...

}