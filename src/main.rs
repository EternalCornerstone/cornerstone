mod setup; // Import the setup module
use std::io;

use crate::setup::setup_check;
use crate::setup::setup_wizard::run_wizard;

fn main() -> io::Result<()> {// Check if the config file exists and is not empty
    
    // more implementations for arguments, could even refactor this into it's own enum impl

    if std::env::args().any(|arg| arg == "--setup-wizard") {
        return run_wizard();
    } else {
        return setup_check();
    }
    
}
