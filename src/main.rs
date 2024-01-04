mod setup; // Import the setup module
use std::io;

use crate::setup::setup_check;

fn main() -> io::Result<()> {// Check if the config file exists and is not empty
    if std::env::args().any(|arg| arg == "--setup-wizard") {
        return setup_check();
    } else {
        println!("No need to setup this wizard.");
    }

    // Continue with the rest of your application logic
    println!("Success!");
    setup_check()?;
    Ok(())
}
