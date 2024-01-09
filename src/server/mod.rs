// pub mod actix;
// pub mod cornerstone;
#[derive(Debug)]
pub enum Server {
    Actix,
    Cornerstone,
    Nothing
}
impl Server {
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
}
