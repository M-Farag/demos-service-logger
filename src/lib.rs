use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

#[derive(Debug)]
pub struct Config {
    file_path:String,
    message:String,
}

impl Config {
    pub fn new() -> Result<Config,Box<dyn Error>> {
        let args:Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("Not enough arguments".into());
        }
        let file_path = args[1].clone();
        let message = args[2].clone();
        Ok(Config {file_path,message})
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn write_to_file(&self) -> Result<(),Box<dyn Error>>
     {
        let mut file_1 = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(self.file_path())?;

        writeln!(file_1,"{}",self.message())?;
        Ok(())
    }
}