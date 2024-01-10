use cornerstone::{run_wizard, setup_check, run_configuration_setup, start_application_with_configuration, error::AppError};


#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Create necessary directories
    std::fs::create_dir_all("./tmp").map_err(AppError::IoError)?;

    if std::env::args().any(|arg: String| arg == "--setup-wizard") {
        run_wizard()?;
    } else {
        setup_check()?;
        let configuration = run_configuration_setup()?;
        start_application_with_configuration(configuration).await?;
    }
    Ok(())
}