#[derive(RustcEncodable, RustcDecodable)]
pub struct LexicalUnit {
    pub surfaces: Vec<String>,
    pub symbols: Vec<String>
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Entry {
    pub l: LexicalUnit,
    pub r: LexicalUnit
}
