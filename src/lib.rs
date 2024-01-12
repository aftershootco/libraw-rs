#[macro_use]
pub mod error;
pub mod dcraw;
pub mod image;
pub mod io;
pub mod processor;
pub use io::{LibrawBufferedDatastream, LibrawDatastream, MaybeDebug};
use processor::Processor;

// pub mod dcraw;
// pub mod defaults;
// #[cfg(feature = "exif")]
// pub mod exif;
// pub mod orientation;
// pub mod progress;
pub mod traits;

pub use error::LibrawError;

extern crate alloc;
extern crate libraw_sys as sys;
use core::ptr::NonNull;
use semver::Version;
use std::ffi::CString;
use std::mem::ManuallyDrop;
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

impl EmptyProcessor {
    pub fn new() -> Result<Self, LibrawError> {
        let inner = unsafe { sys::libraw_init(0) };
        if inner.is_null() {
            return Err(LibrawError::CustomError(
                "libraw_init returned null".to_string().into(),
            ));
        }
        Ok(Self {
            inner: unsafe { NonNull::new_unchecked(inner) },
        })
    }

    pub fn open<'reader, T: std::io::BufRead + std::io::Seek + MaybeDebug + 'reader>(
        mut self,
        reader: T,
    ) -> Result<Processor<'reader>, LibrawError> {
        let mut io = io::LibrawOpaqueDatastream::new(reader);
        let ret = unsafe {
            io::bindings::libraw_open_io(self.inner.as_mut(), core::ptr::addr_of_mut!(io))
        };
        LibrawError::check(ret)?;
        let mut p = unsafe { Processor::new(self.inner) };
        let _ = ManuallyDrop::new(io);
        // p.unpack()?;
        Ok(p)
    }
}
