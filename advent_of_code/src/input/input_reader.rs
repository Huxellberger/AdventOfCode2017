use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::vec::Vec;

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

pub fn read_file_by_line(in_file_location: &str) -> Vec<String>
{
    let string_in = read_file(in_file_location);

    let mut out_lines = Vec::new();

    for line in string_in.lines()
    {
        out_lines.push(line.to_owned());
    }
    
    out_lines
}

pub fn convert_to_numerical_value(in_string: &str) -> i32
{
    let output = in_string.trim().parse::<i32>();

    output.unwrap()
}
