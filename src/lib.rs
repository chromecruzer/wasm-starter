mod utils;

use wasm_bindgen::prelude::*;
use csv::Reader;
use std::io::Cursor;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

#[wasm_bindgen]
pub fn parse_csv(csv_content: &str) -> String {
    let mut rdr = Reader::from_reader(Cursor::new(csv_content));
    let mut output = String::new();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                // Convert the record to a JSON-like string
                let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                output.push_str(&format!("{:?}\n", row));
            }
            Err(err) => {
                return format!("Error parsing CSV: {}", err);
            }
        }
    }

    output
}