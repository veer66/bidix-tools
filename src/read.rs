#[macro_use]
extern crate clap;
extern crate rquery;

use clap::App;
use rquery::{Document, Element};

struct LexicalUnit {
    surfaces: Vec<String>,
    symbols: Vec<String>
}

struct Entry {
    l: LexicalUnit,
    r: LexicalUnit
}

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
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_path = matches.value_of("input").unwrap();
    let doc = Document::new_from_xml_file(input_path).unwrap();

    
    let entries: Vec<Entry> = doc
        .select_all("e")
        .unwrap()
        .map(build_entry)
        .collect();
}
