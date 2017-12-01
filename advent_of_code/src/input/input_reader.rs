use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn read_file(in_file_location: &str) -> String
{
    let path = Path::new(in_file_location);
    let mut open_file = match File::open(path)
    {
        Err(information) => panic!("Couldn't open file {}", Error::description(&information)),
        Ok(open_file) => open_file
    };

    let mut file_body = String::new();
    match open_file.read_to_string(&mut file_body)
    {
        Err(information) => panic!("Couldn't open file {}", Error::description(&information)),
        Ok(_) => {}
    };

    file_body
}
