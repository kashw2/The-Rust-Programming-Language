use std::fs::File;
use std::{error, io};
use std::io::{Error, ErrorKind, Read};

fn main() -> Result<(), Box<dyn error::Error>> {
    let file: Result<File, Error> = File::open("hello.txt");

    match file {
        Ok(file) => dbg!(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:#?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:#?}", other_error);
            }
        },
    };

    println!("{}", read_file_or_create("hi.txt").unwrap_or(String::from("Failed to open hi.txt")));

    let propagated_file = File::open("hi.txt")?;

    Ok(())
}

fn read_file_or_create(name: &str) -> Result<String, io::Error> {
    let non_existing_file: Result<File, Error> = File::open(name);


    let mut file: File = match non_existing_file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut temp_buffer = String::new();

    return match file.read_to_string(&mut temp_buffer) {
        Ok(_) => Ok(temp_buffer),
        Err(error) => Err(error)
    };
}

