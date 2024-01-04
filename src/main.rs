// src/main.rs
use std::{env, io, path::Path};

mod config_setup;
mod setup_wizard;
fn main() -> io::Result<()> {
    // Check if the configuration file exists
    let args: Vec<String> = env::args().collect();
    let config_path = "config.toml";
    // Check for a specific command-line argument to run the setup wizard
    if args.len() > 1 && args[1] == "--setup-wizard" {
        // Call a function to run the setup wizard
        // (Assuming you have a function like this in the setup_wizard module)
        setup_wizard::run_wizard()?;
    } else if Path::new(config_path).exists() {
        // Regular application startup using existing configuration
        config_setup::run_configuration_setup()?;
    } else {
        println!("Configuration file not found. Please run the setup wizard first.");
        return Ok(());
    }
    Ok(())
}