use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Stderr, Stdout};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn guess_vcpkg_dir(base: &Path) -> Option<PathBuf> {
    if let Some(index) = base
        .components()
        .position(|c| c == Component::Normal(OsStr::new("target")))
    {
        let guess = base
            .components()
            .take(index + 1)
            .collect::<PathBuf>()
            .join("vcpkg");

        if guess.read_dir().ok()?.filter_map(|p| p.ok()).any(|p| {
            p.path()
                .file_name()
                .map(|f| f == ".vcpkg-root")
                .unwrap_or_default()
        }) {
            Some(guess)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_vcpkg_triplet(target: &str) -> &'static str {
    match target {
        "x86_64-apple-darwin" => "x64-osx",
        "aarch64-apple-darwin" => "arm64-osx",
        "x86_64-unknown-linux-gnu" => "x64-linux",
        "x86_64-pc-windows-msvc" => "x64-windows-static-md",
        "x86_64-pc-windows-gnu" => "x64-mingw-static",
        "wasm32-unknown-emscripten" => "wasm32-emscripten",
        &_ => panic!("Unsupported target {}", target),
    }
}

fn vcpkg_install_command(vcpkg_root: &PathBuf, vcpkg_triplet: &str, manifest_dir: &str) -> Command {
    let mut x = vcpkg_root.clone();
    if cfg!(windows) {
        x.push("vcpkg.exe");
    } else {
        x.push("vcpkg")
    }
    #[cfg(target_family = "windows")]
    {
        if let Err(_) = std::env::var("GIT_SSH") {
            let default_ssh = PathBuf::from("C:/Windows/System32/OpenSSH/ssh.exe");
            eprintln!(
                "GIT_SSH not set. Checking for existence at {:?}",
                default_ssh
            );
            if default_ssh.exists() {
                println!("SSH agent found at {:?}", default_ssh);
            }
            std::env::set_var("GIT_SSH", default_ssh);
        }
    }
    let mut command = Command::new(x);
    command.current_dir(&manifest_dir);
    command.arg("--triplet");
    command.arg(vcpkg_triplet);
    command.arg("install");
    command
}

fn vcpkg_install(out_dir: &Path) -> Result<String> {
    let vcpkg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vcpkg");
    if !vcpkg_path.exists() {
        let url = "https://github.com/microsoft/vcpkg";
        git2::Repository::clone(url, &vcpkg_path)?;
    }

    let vcpkg_binary = {
        if cfg!(target_family = "windows") {
            PathBuf::from("vcpkg").with_extension("exe")
        } else {
            PathBuf::from("vcpkg")
        }
    };

    if !vcpkg_path.join(vcpkg_binary).exists() {
        let mut install_script = PathBuf::from("bootstrap-vcpkg");
        #[cfg(target_family = "windows")]
        install_script.set_extension("bat");
        #[cfg(target_family = "unix")]
        install_script.set_extension("sh");
        Command::new(vcpkg_path.join(install_script))
            .arg("-disableMetrics")
            .output()
            .unwrap();
    }
    let target = std::env::var("TARGET").unwrap();
    let triplet = get_vcpkg_triplet(&target);
    let mut command = vcpkg_install_command(&vcpkg_path, triplet, env!("CARGO_MANIFEST_DIR"));
    command.arg(&format!(
        "--x-install-root={}/vcpkg_installed",
        out_dir.display()
    ));
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());

    let output = command.spawn().unwrap().wait_with_output().unwrap();

    if !output.status.success() {
        return Err("failed vcpkg install".into());
    }

    println!(
        "cargo:rustc-link-search=native={}/vcpkg_installed/{}/lib",
        out_dir.display(),
        triplet
    );

    Ok(format!(
        "{}/vcpkg_installed/{}/include",
        out_dir.display(),
        triplet
    ))
}

fn apply_patch(libraw_dir: impl AsRef<Path>) -> Result<()> {
    let mut command = Command::new("git");
    command.current_dir(&libraw_dir);
    command.arg("apply");
    command.arg(format!(
        "{}/Add_IntoPix_decoder_and_other_fixes.patch",
        env!("CARGO_MANIFEST_DIR")
    ));
    command.arg("--ignore-whitespace");

    if let Ok(output) = command.output() {
        println!(
            "--- stdout ---\n{}\n\n--- stderr ---\n{}\n\n",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        );
        Ok(())
    } else {
        Err("git apply failed".into())
    }
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LIBRAW_DIR");

    let _out_dir = &std::env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(_out_dir);

    let libraw_dir = std::env::var("LIBRAW_DIR")
        .ok()
        .and_then(|p| {
            shellexpand::full(&p)
                .ok()
                .and_then(|p| dunce::canonicalize(p.to_string()).ok())
        })
        .unwrap_or(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/vendor"
        )));

    // println!("cargo:rerun-if-changed={}", libraw_dir.display());

    println!(
        "cargo:include={}",
        std::env::join_paths([
            Path::new(&libraw_dir).join("libraw"),
            Path::new(&libraw_dir).to_path_buf()
        ])
        .expect("Display")
        .to_string_lossy()
    );

    let vcpkg_include_dir = vcpkg_install(out_dir)?;

    apply_patch(&libraw_dir)?;

    build(out_dir, &libraw_dir, &vcpkg_include_dir)?;

    #[cfg(all(feature = "bindgen"))]
    bindings(out_dir, &libraw_dir)?;

    let _ = out_dir;

    Ok(())
}

fn build(
    out_dir: impl AsRef<Path>,
    libraw_dir: impl AsRef<Path>,
    vcpkg_include_dir: &str,
) -> Result<()> {
    std::env::set_current_dir(out_dir.as_ref()).expect("Unable to set current dir");

    let mut libraw = cc::Build::new();
    libraw.cpp(true);
    libraw.include(libraw_dir.as_ref());

    // Fix builds on msys2
    #[cfg(windows)]
    libraw.define("LIBRAW_WIN32_DLLDEFS", None);
    #[cfg(windows)]
    libraw.define("LIBRAW_BUILDLIB", None);

    // libraw.files(sources);
    // if Path::new("libraw/src/decoders/pana8.cpp").exists() {
    //     libraw.file("libraw/src/decoders/pana8.cpp");
    // }
    // if Path::new("libraw/src/decoders/sonycc.cpp").exists() {
    //     libraw.file("libraw/src/decoders/sonycc.cpp");
    // }
    // if Path::new("libraw/src/decompressors/losslessjpeg.cpp").exists() {
    //     libraw.file("libraw/src/decompressors/losslessjpeg.cpp");
    // }

    let mut sources = vec![
        "src/decoders/canon_600.cpp",
        "src/decoders/crx.cpp",
        "src/decoders/decoders_dcraw.cpp",
        "src/decoders/decoders_libraw.cpp",
        "src/decoders/decoders_libraw_dcrdefs.cpp",
        "src/decoders/dng.cpp",
        "src/decoders/fp_dng.cpp",
        "src/decoders/fuji_compressed.cpp",
        "src/decoders/generic.cpp",
        "src/decoders/kodak_decoders.cpp",
        "src/decoders/load_mfbacks.cpp",
        "src/decoders/olympus14.cpp",
        "src/decoders/pana8.cpp",
        "src/decoders/smal.cpp",
        "src/decoders/sonycc.cpp",
        "src/decoders/unpack.cpp",
        "src/decoders/unpack_thumb.cpp",
        "src/decompressors/losslessjpeg.cpp",
        "src/demosaic/aahd_demosaic.cpp",
        "src/demosaic/ahd_demosaic.cpp",
        "src/demosaic/dcb_demosaic.cpp",
        "src/demosaic/dht_demosaic.cpp",
        "src/demosaic/misc_demosaic.cpp",
        "src/demosaic/xtrans_demosaic.cpp",
        "src/integration/dngsdk_glue.cpp",
        "src/integration/rawspeed_glue.cpp",
        "src/libraw_c_api.cpp",
        "src/libraw_datastream.cpp",
        "src/metadata/adobepano.cpp",
        "src/metadata/canon.cpp",
        "src/metadata/ciff.cpp",
        "src/metadata/cr3_parser.cpp",
        "src/metadata/epson.cpp",
        "src/metadata/exif_gps.cpp",
        "src/metadata/fuji.cpp",
        "src/metadata/hasselblad_model.cpp",
        "src/metadata/identify.cpp",
        "src/metadata/identify_tools.cpp",
        "src/metadata/kodak.cpp",
        "src/metadata/leica.cpp",
        "src/metadata/makernotes.cpp",
        "src/metadata/mediumformat.cpp",
        "src/metadata/minolta.cpp",
        "src/metadata/misc_parsers.cpp",
        "src/metadata/nikon.cpp",
        "src/metadata/normalize_model.cpp",
        "src/metadata/olympus.cpp",
        "src/metadata/p1.cpp",
        "src/metadata/pentax.cpp",
        "src/metadata/samsung.cpp",
        "src/metadata/sony.cpp",
        "src/metadata/tiff.cpp",
        "src/postprocessing/aspect_ratio.cpp",
        "src/postprocessing/dcraw_process.cpp",
        "src/postprocessing/mem_image.cpp",
        "src/postprocessing/postprocessing_aux.cpp",
        "src/postprocessing/postprocessing_utils.cpp",
        "src/postprocessing/postprocessing_utils_dcrdefs.cpp",
        "src/preprocessing/ext_preprocess.cpp",
        "src/preprocessing/raw2image.cpp",
        "src/preprocessing/subtract_black.cpp",
        "src/tables/cameralist.cpp",
        "src/tables/colorconst.cpp",
        "src/tables/colordata.cpp",
        "src/tables/wblists.cpp",
        "src/utils/curves.cpp",
        "src/utils/decoder_info.cpp",
        "src/utils/init_close_utils.cpp",
        "src/utils/open.cpp",
        "src/utils/phaseone_processing.cpp",
        "src/utils/read_utils.cpp",
        "src/utils/thumb_utils.cpp",
        "src/utils/utils_dcraw.cpp",
        "src/utils/utils_libraw.cpp",
        "src/write/apply_profile.cpp",
        "src/write/file_write.cpp",
        "src/write/tiff_writer.cpp",
        "src/x3f/x3f_parse_process.cpp",
        "src/x3f/x3f_utils_patched.cpp",
        //"src/libraw_cxx.cpp"
        //"src/postprocessing/postprocessing_ph.cpp",
        //"src/preprocessing/preprocessing_ph.cpp"
        //"src/write/write_ph.cpp"
    ];

    // Don't set if emscripten as rawspeed doesn't build on wasm yet
    #[cfg(any(
        all(
            not(target_arch = "wasm32"),
            not(target_os = "unknown"),
            feature = "openmp"
        ),
        target_os = "windows"
    ))]
    {
        sources.push("RawSpeed3/rawspeed3_c_api/rawspeed3_capi.cpp");
        sources.push("../src/rawspeed_cameras.cpp");
    }

    let sources = sources
        .iter()
        .filter_map(|s| dunce::canonicalize(libraw_dir.as_ref().join(s)).ok())
        .collect::<Vec<_>>();

    if sources.is_empty() {
        panic!("Sources not found. Maybe try running \"git submodule update --init --recursive\"?");
    } else {
        sources
            .iter()
            .for_each(|s| println!("cargo:rerun-if-changed={}", s.display()));
    }

    libraw.files(sources);

    libraw.include(vcpkg_include_dir);
    libraw.include(format!("{}/dng_sdk", vcpkg_include_dir));
    libraw.include(format!("{}/rawspeed", vcpkg_include_dir));
    libraw.include(format!("{}/rawspeed/external", vcpkg_include_dir));
    libraw.include(format!("{}/IpxCpuCodec", vcpkg_include_dir));
    libraw.include(format!(
        "{}/RawSpeed3/rawspeed3_c_api",
        libraw_dir.as_ref().display()
    ));
    libraw.include(format!(
        "{}/RawSpeed3/rawspeed3_c_api",
        libraw_dir.as_ref().display()
    ));

    #[cfg(unix)]
    {
        libraw.flag_if_supported("-std=c++20");
    }
    #[cfg(windows)]
    {
        libraw.flag_if_supported("/std:c++20");
        libraw.flag_if_supported("/Zc:preprocessor");
        libraw.flag_if_supported("/EHsc");
    }

    libraw.warnings(false);
    libraw.extra_warnings(false);
    // do I really have to supress all of these?
    libraw.flag_if_supported("-Wno-format-truncation");
    libraw.flag_if_supported("-Wno-unused-result");
    libraw.flag_if_supported("-Wno-format-overflow");
    #[cfg(feature = "openmp")]
    {
        libraw.define("LIBRAW_FORCE_OPENMP", None);
        std::env::var("DEP_OPENMP_FLAG")
            .unwrap()
            .split(' ')
            .for_each(|f| {
                libraw.flag(f);
            });
        if cfg!(target_os = "macos") {
            if libraw.get_compiler().is_like_apple_clang() {
                let homebrew_prefix =
                    PathBuf::from(std::env::var("HOMEBREW_PREFIX").unwrap_or_else(|_| {
                        if cfg!(target_arch = "aarch64") {
                            "/opt/homebrew".into()
                        } else {
                            "/usr/local".into()
                        }
                    }));

                if homebrew_prefix.join("opt/libomp/include").exists() {
                    libraw.include(homebrew_prefix.join("opt/libomp/include"));
                    println!(
                        "cargo:rustc-link-search={}{}opt/libomp/lib",
                        homebrew_prefix.display(),
                        std::path::MAIN_SEPARATOR
                    );
                    let statik = cfg!(feature = "openmp_static");
                    println!(
                        "cargo:rustc-link-lib{}=omp",
                        if statik { "=static" } else { "" }
                    );
                } else {
                    println!("cargo:warning:Unable to find libomp (maybe try installing libomp via homebrew?)")
                }
            }
        }
    }
    // thread safety
    libraw.flag_if_supported("-pthread");

    // Add libraries
    libraw.flag("-DUSE_DNGSDK");

    // Don't set if emscripten as rawspeed doesn't build on wasm yet
    #[cfg(any(
        all(
            not(target_arch = "wasm32"),
            not(target_os = "unknown"),
            feature = "openmp"
        ),
        target_os = "windows"
    ))]
    {
        libraw.flag("-DRAWSPEED_BUILDLIB");
        libraw.flag("-DUSE_RAWSPEED3");
    }

    libraw.flag("-DUSE_X3FTOOLS");

    libraw.flag("-DUSE_JPEG");
    libraw.flag("-DUSE_JPEG8");

    libraw.flag("-DUSE_ZLIB");

    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "unknown")))]
    {
        libraw.flag("-DUSE_INTOPIX_CPU_CODEC");
        libraw.flag("-DIPXCPUCODEC_LIB_STATIC");
    }

    libraw.compile("raw_r");

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.as_ref().join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=raw_r");

    #[cfg(any(
        all(
            not(target_arch = "wasm32"),
            not(target_os = "unknown"),
            feature = "openmp"
        ),
        target_os = "windows"
    ))]
    println!("cargo:rustc-link-lib=static=rawspeed");
    println!("cargo:rustc-link-lib=static=dng");
    println!("cargo:rustc-link-lib=static=jxl_threads");
    println!("cargo:rustc-link-lib=static=jxl");
    println!("cargo:rustc-link-lib=static=jxl_cms");
    println!("cargo:rustc-link-lib=static=lcms2");
    println!("cargo:rustc-link-lib=static=hwy");
    println!("cargo:rustc-link-lib=static=brotlidec");
    println!("cargo:rustc-link-lib=static=brotlienc");
    println!("cargo:rustc-link-lib=static=brotlicommon");
    println!("cargo:rustc-link-lib=static=jpeg");
    println!("cargo:rustc-link-lib=static=pugixml");
    println!("cargo:rustc-link-lib=static=XMP");

    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "unknown")))]
    {
        println!("cargo:rustc-link-lib=static=IpxCpuCodec_static");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=static=expat");
        println!("cargo:rustc-link-lib=static=png16");
        println!("cargo:rustc-link-lib=static=z");
    }
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=static=expat");
        println!("cargo:rustc-link-lib=static=png16");
        println!("cargo:rustc-link-lib=static=z");
    }
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=static=libexpatMD");
        println!("cargo:rustc-link-lib=static=libpng16");
        println!("cargo:rustc-link-lib=static=zlib");
        // Needed by IntoPix
        println!("cargo:rustc-link-lib=iphlpapi");
    }

    // Needed for libgomp linking on Linux, otherwise undefined references are thrown
    #[cfg(target_os = "linux")]
    if let Some(link) = std::env::var_os("DEP_OPENMP_CARGO_LINK_INSTRUCTIONS") {
        for i in std::env::split_paths(&link) {
            println!("cargo:{}", i.display());
        }
    }

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    Ok(())
}

#[cfg(feature = "bindgen")]
fn bindings(out_dir: impl AsRef<Path>, libraw_dir: impl AsRef<Path>) -> Result<()> {
    let bindings = bindgen::Builder::default()
        .header(
            libraw_dir
                .as_ref()
                .join("libraw")
                .join("libraw.h")
                .to_string_lossy(),
        )
        .use_core()
        .ctypes_prefix("libc")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // API improvements
        .derive_eq(true)
        .size_t_is_usize(true)
        // these are never part of the API
        .blocklist_function("_.*")
        // consts creating duplications
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("__mingw_ldbl_type_t")
        // Rust doesn't support long double, and bindgen can't skip it
        // https://github.com/rust-lang/rust-bindgen/issues/1549
        .blocklist_function("acoshl")
        .blocklist_function("acosl")
        .blocklist_function("asinhl")
        .blocklist_function("asinl")
        .blocklist_function("atan2l")
        .blocklist_function("atanhl")
        .blocklist_function("atanl")
        .blocklist_function("cbrtl")
        .blocklist_function("ceill")
        .blocklist_function("copysignl")
        .blocklist_function("coshl")
        .blocklist_function("cosl")
        .blocklist_function("dreml")
        .blocklist_function("ecvt_r")
        .blocklist_function("erfcl")
        .blocklist_function("erfl")
        .blocklist_function("exp2l")
        .blocklist_function("expl")
        .blocklist_function("expm1l")
        .blocklist_function("fabsl")
        .blocklist_function("fcvt_r")
        .blocklist_function("fdiml")
        .blocklist_function("finitel")
        .blocklist_function("floorl")
        .blocklist_function("fmal")
        .blocklist_function("fmaxl")
        .blocklist_function("fminl")
        .blocklist_function("fmodl")
        .blocklist_function("frexpl")
        .blocklist_function("gammal")
        .blocklist_function("hypotl")
        .blocklist_function("ilogbl")
        .blocklist_function("isinfl")
        .blocklist_function("isnanl")
        .blocklist_function("j0l")
        .blocklist_function("j1l")
        .blocklist_function("jnl")
        .blocklist_function("ldexpl")
        .blocklist_function("lgammal")
        .blocklist_function("lgammal_r")
        .blocklist_function("llrintl")
        .blocklist_function("llroundl")
        .blocklist_function("log10l")
        .blocklist_function("log1pl")
        .blocklist_function("log2l")
        .blocklist_function("logbl")
        .blocklist_function("logl")
        .blocklist_function("lrintl")
        .blocklist_function("lroundl")
        .blocklist_function("modfl")
        .blocklist_function("nanl")
        .blocklist_function("nearbyintl")
        .blocklist_function("nextafterl")
        .blocklist_function("nexttoward")
        .blocklist_function("nexttowardf")
        .blocklist_function("nexttowardl")
        .blocklist_function("powl")
        .blocklist_function("qecvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qgcvt")
        .blocklist_function("remainderl")
        .blocklist_function("remquol")
        .blocklist_function("rintl")
        .blocklist_function("roundl")
        .blocklist_function("scalbl")
        .blocklist_function("scalblnl")
        .blocklist_function("scalbnl")
        .blocklist_function("significandl")
        .blocklist_function("sinhl")
        .blocklist_function("sincosl")
        .blocklist_function("sinl")
        .blocklist_function("sqrtl")
        .blocklist_function("strtold")
        .blocklist_function("tanhl")
        .blocklist_function("tanl")
        .blocklist_function("tgammal")
        .blocklist_function("truncl")
        .blocklist_function("wcstold")
        .blocklist_function("y0l")
        .blocklist_function("y1l")
        .blocklist_function("ynl")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.as_ref().join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(feature = "copy")]
    bindings
        .write_to_file(
            #[cfg(target_os = "linux")]
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("linux.rs"),
            #[cfg(target_os = "macos")]
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("macos.rs"),
            #[cfg(target_family = "windows")]
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("windows.rs"),
        )
        .expect("Failed to write bindings");
    Ok(())
}

pub trait IsAppleClang {
    fn try_is_like_apple_clang(&self) -> Result<bool>;
    fn is_like_apple_clang(&self) -> bool {
        self.try_is_like_apple_clang()
            .expect("Failed to run compiler")
    }
}

impl IsAppleClang for cc::Tool {
    fn try_is_like_apple_clang(&self) -> Result<bool> {
        let output = std::process::Command::new(self.to_command().get_program())
            .arg("-v")
            .output()?;
        let stderr = String::from_utf8(output.stderr)?;
        Ok(stderr.starts_with("Apple") && (stderr.contains("clang") || self.is_like_clang()))
    }
}
