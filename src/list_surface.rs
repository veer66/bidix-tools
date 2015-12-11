#[macro_use]
extern crate clap;
extern crate xml;
extern crate rustc_serialize;

mod data;
mod reader;

use clap::App;
use reader::read_bidix;

fn main() {
    let yaml = load_yaml!("list_surface.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_path = matches.value_of("input").unwrap();
    let pos = matches.value_of("pos").unwrap();
    let entries = read_bidix(input_path);

    for entry in entries.iter() {
        let lu = match pos {
            "left" => &entry.l,
            "right" => &entry.r,
            _ => panic!("Invalid pos: {}", pos)
        };
        if lu.surfaces.len() > 0 {
            println!("{}", entry.l.surfaces[0]);
        }
    }
}
