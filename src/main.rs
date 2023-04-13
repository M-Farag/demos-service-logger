use std::fs;
use std::io::Write;
use demos_service_logger::*;

fn main() {
    // Todo
    // [x] Creating a lib module
    // [x] Migrate the env args functionality to the lib module
    // [x] Make sure that the lib module is able to handle the arguments
    // [] Migrate the file handler to the lib module
    // [] Make sure that the lib module is able to handle the file
    // [] Error handling
    // [] Testing that everything works

    

    let config = Config::new().unwrap_or_else(
        |err| {
            println!("Err: {}",err);
            std::process::exit(1);
        }
    );

    config.write_to_file().unwrap()


    
}
