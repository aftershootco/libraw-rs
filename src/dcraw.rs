use crate::image::*;
use crate::*;
impl Processor<'_> {
    pub fn dcraw_process_make_mem_thumb(&mut self) -> Result<ProcessedImage, LibrawError> {
        let mut errc = 0;
        let data = unsafe { sys::libraw_dcraw_make_mem_thumb(self.inner.as_ptr(), &mut errc) };
        assert!(!data.is_null());
        LibrawError::to_result(
            errc,
            ProcessedImage {
                inner: NonNull::new(data).expect("Not Null"),
            },
        )
    }

    pub fn dcraw_process(&mut self) -> Result<(), LibrawError> {
        LibrawError::check(unsafe { sys::libraw_dcraw_process(self.inner.as_ptr()) })?;
        Ok(())
    }

    pub fn dcraw_process_make_mem_image(&mut self) -> Result<ProcessedImage, LibrawError> {
        let mut errc = 0;
        let data = unsafe { sys::libraw_dcraw_make_mem_image(self.inner.as_ptr(), &mut errc) };
        assert!(!data.is_null());
        LibrawError::to_result(
            errc,
            ProcessedImage {
                inner: NonNull::new(data).expect("Not null"),
            },
        )
    }

    pub fn dcraw_ppm_tiff_writer(
        self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), LibrawError> {
        LibrawError::check(unsafe {
            sys::libraw_dcraw_ppm_tiff_writer(self.inner.as_ptr(), path_to_cstr(path)?.as_ptr())
        })?;
        Ok(())
    }
}

#[cfg(unix)]
fn path_to_cstr(path: impl AsRef<Path>) -> Result<CString, std::ffi::NulError> {
    use std::os::unix::ffi::OsStrExt;
    let path = path.as_ref().as_os_str().as_bytes();
    CString::new(path)
}
#[cfg(windows)]
fn path_to_cstr(path: impl AsRef<Path>) -> Result<CString, std::ffi::NulError> {
    let path = path.as_ref().display().to_string();
    let path = path.as_bytes();
    CString::new(path)
}
