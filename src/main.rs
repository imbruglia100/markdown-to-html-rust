use std::io;
use std::fs::File;
use conversion::convert;
use std::io::prelude::*;
mod conversion;
//import covert function from converter.rs


fn read_file() -> Result<String, io::Error> {
    let f = File::open("example.md");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    if read_file().is_err() {
        return println!("Error reading file");
    } else {
        convert(&read_file().unwrap());
    }
}
