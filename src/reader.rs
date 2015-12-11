use data::{Entry, LexicalUnit};
use std::fs::File;
use std::io::BufReader;
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

pub fn read_bidix(path: &str) -> Vec<Entry> {
    let input_file = File::open(path).unwrap();
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

    return entries
}
