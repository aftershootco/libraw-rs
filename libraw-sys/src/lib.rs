#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::approx_constant)]
#![allow(clippy::too_many_arguments)]
#[cfg(feature = "openmp")]
extern crate openmp_sys;

#[cfg(all(windows, target_env = "msvc", not(feature = "bindgen")))]
#[path = "windows.rs"]
mod bindings;

#[cfg(all(windows, target_env = "gnu", not(feature = "bindgen")))]
#[path = "windows_gnu.rs"]
mod bindings;

#[cfg(all(target_os = "macos", not(feature = "bindgen")))]
#[path = "macos.rs"]
mod bindings;

#[cfg(all(target_os = "linux", not(feature = "bindgen")))]
#[path = "linux.rs"]
mod bindings;

#[cfg(all(target_family = "wasm", not(feature = "bindgen")))]
mod bindings {
    compile_error!("WebAssembly is not supported without bindgen");
    compile_error!("Please enable the `bindgen` feature to generate the bindings");
}

#[cfg(feature = "bindgen")]
mod bindings;

pub use self::bindings::*;
