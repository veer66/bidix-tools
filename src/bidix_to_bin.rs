#[macro_use]
extern crate clap;
extern crate rquery;
extern crate bincode;
extern crate rustc_serialize;

mod data;

use clap::App;
use rquery::{Document, Element};
use bincode::SizeLimit;
use bincode::rustc_serialize::encode;
use std::fs::File;
use std::io::Write;
use data::{Entry, LexicalUnit};
fn build_lexical_unit(unit: &Element) -> LexicalUnit {
    let mut surfaces: Vec<String> = vec![];
    surfaces.push(unit.text().clone());
    match unit.select("g") {
        Err(_) => {},
        Ok(g) => {
            surfaces.push(g.text().clone())
        }
    }
    let symbols: Vec<String> = unit
        .select_all("s")
        .unwrap()
        .map(|sym| sym.attr("n").unwrap().clone())
        .collect();
    
    LexicalUnit{surfaces: surfaces,
                symbols: symbols}
}

fn build_entry(element: &Element) -> Entry {
    let p = element.select("p").expect("Cannot find <p>");
    let l = build_lexical_unit(p.select("l").expect("Cannot find <l>"));
    let r = build_lexical_unit(p.select("r").expect("Cannot find <r>"));            
    Entry {l: l, r: r}
}

fn main() {
    let yaml = load_yaml!("bidix_to_bin.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();
    let mut output_file = File::create(output_path)
        .expect("Cannot open file for writing");
    let doc = Document::new_from_xml_file(input_path).unwrap();

    
    let entries: Vec<Entry> = doc
        .select_all("e")
        .unwrap()
        .map(build_entry)
        .collect();

    
    output_file.write(&encode(&entries, SizeLimit::Infinite).unwrap()[..])
        .expect("Cannot write");
}
