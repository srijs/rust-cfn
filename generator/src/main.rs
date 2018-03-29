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
    let mut data = String::new();
    let mut gz = flate2::read::GzDecoder::new(&bytes[..]);
    gz.read_to_string(&mut data)
        .expect("failed to decompress specification file");
    let specification = serde_json::from_str::<model::Specification>(&data)
        .expect("failed to parse specification data");
    codegen::generate(specification, "../src/aws")
        .expect("failed to generate output files");
}
