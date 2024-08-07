fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "emscripten" {
        println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=ccall,cwrap");
        println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");
        println!("cargo:rustc-link-arg=-sEXPORTED_FUNCTIONS=_malloc,_buffet,_free,_vec_drop");
    }
}
