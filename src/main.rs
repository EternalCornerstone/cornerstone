use cornerstone::{api::start_http_server, run_wizard, setup_check, run_configuration_setup, start_application_with_configuration};


#[actix_web::main]
async fn main() {
    // Create necessary directories
    if let Err(e) = std::fs::create_dir_all("./tmp") {
        eprintln!("Failed to create directories: {}", e);
        return;
    }

    if std::env::args().any(|arg: String| arg == "--setup-wizard") {
        match run_wizard() {
            Ok(_) => println!("Setup wizard completed successfully."),
            Err(e) => eprintln!("Error running setup wizard: {}", e),
        }
    } else {
        match setup_check() {
            Ok(_) => println!("Setup check completed."),
            Err(e) => {
                eprintln!("Error during setup check: {}", e);
                return;
            },
        }

        match run_configuration_setup() {
            Ok(configuration) => {
                println!("Starting the application using the specific configuration: ");
                start_application_with_configuration(configuration);
            },
            Err(e) => eprintln!("Error setting up configuration: {}", e),
        }
    }

    if let Err(e) = start_http_server().await {
        eprintln!("Failed to start HTTP server: {}", e);
    }
}