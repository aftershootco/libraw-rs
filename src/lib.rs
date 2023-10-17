#[macro_use]
pub mod error;
pub mod io;
// pub mod dcraw;
// pub mod defaults;
// #[cfg(feature = "exif")]
// pub mod exif;
// pub mod orientation;
// pub mod progress;
// pub mod traits;

use alloc::sync::Arc;
pub use error::LibrawError;

extern crate alloc;
extern crate libraw_sys as sys;
use core::ptr::NonNull;
use core::sync::atomic::AtomicBool;
use semver::Version;
use std::ffi::CString;
use std::ops::Drop;
use std::path::Path;

/// Returns the version of libraw the bindings were generated against
pub const fn bindings() -> Version {
    Version {
        major: sys::LIBRAW_MAJOR_VERSION as u64,
        minor: sys::LIBRAW_MINOR_VERSION as u64,
        patch: sys::LIBRAW_PATCH_VERSION as u64,
        pre: semver::Prerelease::EMPTY,
        build: semver::BuildMetadata::EMPTY,
    }
}

/// An empty processor that can open files
pub struct EmptyProcessor {
    pub(crate) inner: NonNull<sys::libraw_data_t>,
}

