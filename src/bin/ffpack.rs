use ffpack::Pack;

pub fn main() {
    let pack = Pack::default();
    let output = serde_json::to_string_pretty(&pack).unwrap();
    println!("{}", output);
}
