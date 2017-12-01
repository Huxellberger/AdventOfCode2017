use std::fs::File;
use std::path::Path;

pub fn read_file(inFileLocation: &str)
{
    let path = Path::new(inFileLocation);
    let open_file = File::open(path);
}