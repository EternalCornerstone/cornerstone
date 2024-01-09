use std::{io::{self, Write, Error}, fs::{File, self}};

use database::Database;
use server::Server;

pub mod api;
pub mod server;
pub mod database;
pub mod file_upload;
pub mod payment;
pub mod security;
pub mod products;
pub mod ui;

pub fn run_wizard() -> io::Result<()> {
    Server::give_users_server_options();    
    let server_input = unwrap_input();
    let server = server::Server::get_config(&server_input).to_owned();
    println!("input for server {:?}", server);

    // Database feature
    database::Database::give_users_database_options();
    let database_input = unwrap_input();
    println!("input for database: {:?}", &database_input);

    let database = database::Database::get_config(&database_input).to_owned();

    // Create and write to the config file
    let mut config_file = File::create("config.toml")?;
    writeln!(config_file, "server = {:?}", server)?;
    writeln!(config_file, "database = {:?}", database)?;
    println!("Configuration saved to config.toml");

    Ok(())
}
fn unwrap_input() -> String {    
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned() // Trims and returns the input
}

pub fn setup_check() -> io::Result<()> {
    // Check if the config file exists
    if !fs::metadata("config.toml").is_ok() {
        println!("It seems that you are trying to run the project without having a config.toml file, lets fix this!");
        println!("For context, the config.toml will tell the application which package you would like to use within your project.");
        println!("Note that there are mandatory packages to allow for core functionality.");
        // The config file doesn't exist, so run the setup wizard
        if std::env::args().any(|arg| arg == "--setup-wizard") {
            println!("Running setup wizard...");
            run_wizard()?; // Run the setup_wizard module
        } else {
            let mut input = String::new();
            println!("Config file not found. To run the setup wizard");
            println!("If you'd like to do this yourself then you can use: `cargo run -- --setup-wizard`");
            println!("Alternatively we can do this automatically? it's up to you... [y/N]");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let start_setup_wizard = input.trim().eq_ignore_ascii_case("y");
            input.clear();
            if start_setup_wizard {
                println!("Great, lets get started.");
                run_wizard()?;
            } else {
                println!("Okay great I know it's sometimes nice to go through the commands manually to learn the process.");
                println!("The system will now EXIT.");
                println!("Remember to run the command: `cargo run -- --setup-wizard` to successfully build the application with some working features");
                return Ok(())
            }
        }
    } else {
        println!("Wizard has already been setup, config file exists so lets initialize the configuration into the application:)");
    }
    Ok(())
}

pub fn run_configuration_setup() -> Result<Configuration, Error>{
    // Read the configuration file
    let config_contents = fs::read_to_string("config.toml")?;

    // Variables to hold your configuration values
    let mut server = String::new();
    let mut database = String::new();

    // Parse the configuration
    for line in config_contents.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            match parts[0] {
                "server" => server = parts[1].trim_matches('"').to_string(),
                "database" => database = parts[1].trim_matches('"').to_string(),
                _ => continue,
            }
        }
    }

    let config = Configuration {
        server: Server::as_server(&server),
        database: Database::as_database(&database)
    };
    Ok(config)
}

#[derive(Debug)]
pub struct Configuration {
    server: Server,
    database: Database,
}
impl Configuration {
    pub fn new(configuration: Configuration) -> Configuration {
        Configuration {
            server: configuration.server,
            database: configuration.database,
        }
    }

}
pub fn start_application_with_configuration(configuration: Configuration) {
    println!("{:?}", configuration);
}