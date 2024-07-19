use std::io::Write;

fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let buffer = &raw_rendering::unpack(path).unwrap();
    std::io::stdout().write_all(serde_json::json!(buffer).to_string().as_bytes());
}
