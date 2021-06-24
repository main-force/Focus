use focus::{init, run};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    init::file_check();
    
    run()
}
