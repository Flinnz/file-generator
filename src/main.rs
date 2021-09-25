use std::fs::File;
use std::env;
use std::io::Read;
use std::io::{Write, BufWriter};

use rand::prelude::*;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn generate_file_from_random_strings(file_name: String, file_size: usize, strings: &Vec<&str>) -> std::io::Result<()> {
    let mut output_file = BufWriter::new(File::create(file_name)?);
    let mut total_size = 0;
    let mut rng = rand::thread_rng();
    
    while total_size < file_size {
        let n = rng.next_u32() % (strings.len() as u32);
        println!("Writing string number {} to file", n + 1);
        total_size += write_string_to_file(strings[n as usize], &mut output_file)?;
    }
 
    Ok(())
}

fn write_string_to_file(string: &str, file: &mut BufWriter<File>) -> std::io::Result<usize> {
    let output_string_bytes = string.as_bytes();
    let size = output_string_bytes.len();
    file
        .write_all(output_string_bytes)
        .map(|_| -> usize { size } )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: file-generator <target text file>");
        return;
    }
    let filename = &args[1];
    println!("Reading file {}", filename);
    
    let file = File::open(filename)
        .expect("Error opening file");

    let mut contents = String::new();
    DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1251))
        .build(file)
        .read_to_string(& mut contents)
        .expect("Error while reading file");
  
    let strings: Vec<&str> = contents
        .split("\n")
        .collect();
    println!("Strings in file: {}", strings.len());

    let mut output_file_name = filename.to_owned();
    output_file_name.push_str("1MB");
    generate_file_from_random_strings(output_file_name, 1 << 20, &strings)
        .expect("Error writing strings to file");
    println!("done");
}
