fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let buffer = &raw_rendering::unpack(path).unwrap();
    dbg!(buffer);
}
