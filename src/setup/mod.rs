use std::io;
use std::fs;
use std::io::Write;

pub mod setup_wizard;
mod config_setup;

pub fn setup_check() -> io::Result<()> {
    // Check if the config file exists
    if !fs::metadata("config.toml").is_ok() {
        println!("It seems that you are trying to run the project without having a config.toml file, lets fix this!");
        println!("For context, the config.toml will tell the application which package you would like to use within your project.");
        println!("Note that there are mandatory packages to allow for core functionality.");
        // The config file doesn't exist, so run the setup wizard
        if std::env::args().any(|arg| arg == "--setup-wizard") {
            println!("Running setup wizard...");
            setup_wizard::run_wizard()?; // Run the setup_wizard module
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
                setup_wizard::run_wizard()?;
            } else {
                println!("Okay great I know it's sometimes nice to go through the commands manually to learn the process.");
                println!("The system will now EXIT.");
                println!("Remember to run the command: `cargo run -- --setup-wizard` to successfully build the application with some working features");
                return Ok(())
            }
        }
    } else {
        println!("Wizard has already been setup, config file exists so lets initialize the configuration into the application:)");
        config_setup::run_configuration_setup()?;
        println!("Application is configured");
    }

    Ok(())
}