#[macro_use]
extern crate clap;
extern crate bincode;
extern crate rustc_serialize;
extern crate xml;

mod data;
mod reader;

use clap::App;
use bincode::SizeLimit;
use bincode::rustc_serialize::encode;
use std::fs::File;
use std::io::Write;
use reader::read_bidix;

fn main() {
    let yaml = load_yaml!("bidix_to_bin.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();
    let mut output_file = File::create(output_path)
        .expect("Cannot open file for writing");    
    let entries = read_bidix(input_path);
    output_file.write(&encode(&entries, SizeLimit::Infinite).unwrap()[..])
        .expect("Cannot write");
}
