#[macro_use]
extern crate clap;
extern crate bincode;
extern crate rustc_serialize;
extern crate xml;

mod data;

use clap::App;
use bincode::SizeLimit;
use bincode::rustc_serialize::encode;
use std::fs::File;
use std::io::{Write, BufReader};
use data::{Entry, LexicalUnit};
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

fn build_lexical_unit(parser: &mut EventReader<BufReader<File>>,
                      tag: &str) -> LexicalUnit {
    let mut surfaces = vec![];
    let mut symbols = vec![];

    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match &name.local_name[..] {
                    "s" => {
                        let attrs: Vec<&OwnedAttribute> = attributes
                            .iter()
                            .filter(|s| s.name.local_name == "s")
                            .collect();
                        if attrs.len() == 1 {
                            symbols.push(attrs[0].value.to_string());
                        }
                    }
                    "b" => {
                        if surfaces.is_empty() {
                            surfaces.push(" ".to_string());
                        } else {
                            surfaces.last_mut().unwrap().push_str(" ");
                        }
                    }
                    "g" => {
                        surfaces.push("".to_string());
                    }
                    _ => {} 
                }               
            },
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == tag {
                   break;
                }               
            },
            Ok(XmlEvent::Characters(s)) => {
                if surfaces.is_empty() {
                    surfaces.push(s);
                } else {
                    surfaces.last_mut().unwrap().push_str(&s[..]);
                }
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {}
        }
    }
    
    LexicalUnit{surfaces: surfaces,
                symbols: symbols}
}

fn build_entry(parser: &mut EventReader<BufReader<File>>) -> Entry {
    let l = build_lexical_unit(parser, "l");
    let r = build_lexical_unit(parser, "r");
    
    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "e" {
                   break;
                }               
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {}
        }
    }

    Entry {l: l, r: r}
}

fn main() {
    let yaml = load_yaml!("bidix_to_bin.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();
    let mut output_file = File::create(output_path)
        .expect("Cannot open file for writing");
    
    let input_file = File::open(input_path).unwrap();
    let input_buf = BufReader::new(input_file);

    let mut parser = EventReader::new(input_buf);
    let mut entries = vec![];

    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "e" {
                    entries.push(build_entry(&mut parser));
                }               
            }
            Ok(XmlEvent::EndDocument) => {
                break;
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {}
        }
    }
    
    output_file.write(&encode(&entries, SizeLimit::Infinite).unwrap()[..])
        .expect("Cannot write");
}
