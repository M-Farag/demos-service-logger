use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Todo
    // [x] accept arguments
    // [x] define file_path & a message
    // [x] Open a file for appending
    // [x] Write the message to the file
    // [x] check the results

    let args:Vec<String> = env::args().collect();

    let file_path = &args[1];
    let message = &args[2];

    let mut file_1 = fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_path).unwrap();

    writeln!(file_1,"{}",message);


    
}
