use std::path::Path;
fn main() -> anyhow::Result<()> {
    let _out_dir = &std::env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(_out_dir);

    #[cfg(feature = "exif")]
    libread(out_dir)?;

    riio(out_dir)?;

    Ok(())
}

#[cfg(feature = "exif")]
pub fn libread(out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut libread = cc::Build::new();

    let includes = std::env::var("DEP_RAW_R_INCLUDE")?;
    let includes = std::env::split_paths(&includes).collect::<Vec<_>>();
    libread
        .includes(includes)
        .cpp(true)
        .file("exif/libread.cpp")
        .static_flag(true)
        .shared_flag(false);

    #[cfg(windows)]
    libread.static_crt(true);

    libread.compile("read");

    println!("cargo:rustc-link-lib=static=read");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.as_ref().join("lib").display()
    );

    Ok(())
}

pub fn riio(out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=src/io.rs");
    let includes = std::env::var("DEP_RAW_R_INCLUDE")?;
    let includes = std::env::split_paths(&includes).collect::<Vec<_>>();

    cbindgen::Builder::new()
        .with_crate(env!("CARGO_MANIFEST_DIR"))
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_cpp_compat(true)
        .with_header("#include<stdint.h>")
        .with_include_guard("RUST_IO_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("io/riio.h");

    let clang_args = includes
        .iter()
        .map(|p| format!("-I{}", p.to_str().unwrap()))
        .inspect(|p| println!("cargo:warning=clang_args: {}", p));

    let bindings = bindgen::Builder::default()
        .header("io/io.h")
        .allowlist_file("io/io.h")
        .blocklist_type(".*")
        .clang_args(clang_args);

    let triple = std::env::var("TARGET")?;
    let bindings = if triple.ends_with("emscripten") {
        // Check emcc
        let out = std::process::Command::new("emcc")
            .arg("--cflags")
            .output()?;
        let cflags = String::from_utf8(out.stdout)?;
        bindings.clang_args(cflags.split_whitespace())
    } else {
        bindings
    };

    bindings
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(out_dir.as_ref().join("io.rs"))
        .expect("failed to write bindings");

    let mut riio = cc::Build::new();
    riio.includes(includes)
        .cpp(true)
        .file("io/io.cpp")
        .static_flag(true)
        .shared_flag(false);

    #[cfg(windows)]
    riio.static_crt(true);

    riio.compile("riio");

    println!("cargo:rerun-if-changed=io/io.cpp");
    println!("cargo:rerun-if-changed=io/io.h");
    println!("cargo:rustc-link-lib=static=riio");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.as_ref().join("lib").display()
    );

    Ok(())
}
