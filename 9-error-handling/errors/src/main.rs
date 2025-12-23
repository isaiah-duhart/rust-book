use std::fs::{self, File};
use std::io::{self, Read};

fn main() { 
    match read_file_explicit("recipe.txt") {
        Err(error) => println!("Error reading recipe file {}", error),
        Ok(string) => println!("{}", string)
    };
    match read_file_implicit("passwords.txt") {
        Err(error) => println!("Error reading recipe file {}", error),
        Ok(string) => println!("{}", string)
    };
    match read_file_explicit("non-existant.txt") {
        Err(error) => println!("Error reading recipe file {}", error),
        Ok(string) => println!("{}", string)
    };
    match read_file_implicit("non-existant.txt") {
        Err(error) => println!("Error reading recipe file {}", error),
        Ok(string) => println!("{}", string)
    };
    match fs::read_to_string("idiomatic.txt") {
        Err(error) => println!("Error reading file {}", error),
        Ok(string) => println!("{}", string)
    }
    match fs::read_to_string("non-existant.txt") {
        Err(error) => println!("Error reading recipe file {}", error),
        Ok(string) => println!("{}", string)
    };
}

fn read_file_explicit(filename: &str) -> Result<String, io::Error>  {
    let file_result = File::open(filename);
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(err) => return Err(err)
    };

    return Ok(string);
}

fn read_file_implicit(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}
