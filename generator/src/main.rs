extern crate flate2;
extern crate heck;
extern crate itertools;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::io::Read;

mod model;
mod codegen;

fn main() {
    let bytes = include_bytes!("../CloudFormationResourceSpecification.json.gz");
    let mut gz = flate2::read::GzDecoder::new(&bytes[..]);    
    let mut buf = Vec::new();
    gz.read_to_end(&mut buf).unwrap();

    let data = std::str::from_utf8(&buf).unwrap();
    
    let specification = serde_json::from_str::<model::Specification>(&data)
        .expect("failed to parse specification data");
    codegen::generate(specification, "../src/aws")
        .expect("failed to generate output files");
}
