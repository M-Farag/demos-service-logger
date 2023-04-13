use demos_service_logger::*;

fn main() {
    // Todo
    // [x] Creating a lib module
    // [x] Migrate the env args functionality to the lib module
    // [x] Make sure that the lib module is able to handle the arguments
    // [x] Migrate the file handler to the lib module
    // [x] Make sure that the lib module is able to handle the file
    // [x] Error handling
    // [x] Testing that everything works

    

    let config = Config::new().unwrap_or_else(
        |err| {
            println!("Err: {}",err);
            std::process::exit(1);
        }
    );

    if let Err(e) = config.write_to_file() {
        println!("Error: {}",e);
        std::process::exit(1);
    }


    
}
