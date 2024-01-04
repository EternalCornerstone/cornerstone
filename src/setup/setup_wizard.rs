use std::{io::{self, Write}, fs::File};

use crate::setup::config_setup;

pub fn run_wizard() -> io::Result<()> {
    let mut input = String::new();
    println!("Welcome to the Cornerstone Setup Wizard!");

    // Auth feature
    println!("Do you want to enable the 'auth' feature? [y/N]");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let auth_enabled = input.trim().eq_ignore_ascii_case("y");
    input.clear();

    // Database feature
    println!("Do you want to enable the 'database' feature? [y/N]");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let database_enabled = input.trim().eq_ignore_ascii_case("y");

    // Create and write to the config file
    let mut config_file = File::create("./config.toml")?;
    writeln!(config_file, "auth_enabled = {}", auth_enabled)?;
    writeln!(config_file, "database_enabled = {}", database_enabled)?;

    println!("Configuration saved to config.toml");

    
    println!("Setup complete! Auth enabled: {}, Database enabled: {}", auth_enabled, database_enabled);

    println!("Applying the configuration to the application.");

    config_setup::run_configuration_setup()?;

    Ok(())
}