#[macro_use]
extern crate clap;
extern crate bincode;
extern crate rustc_serialize;

mod data;

use clap::App;
use bincode::rustc_serialize::decode;
use data::Entry;
use std::fs::File;
use std::io::Read;

fn main() {
    let yaml = load_yaml!("list_surface.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_path = matches.value_of("input").unwrap();
    let mut input_file = File::open(input_path)
        .expect("Cannot open file for reading");
    let mut buf = vec![];
    input_file
        .read_to_end(&mut buf)
        .expect("Cannot read");
    let entries: Vec<Entry> = decode(&buf[..])
        .expect("Cannot decode");

    for entry in entries.iter() {
        println!("{}", entry.l.surfaces[0]);
    }
}
