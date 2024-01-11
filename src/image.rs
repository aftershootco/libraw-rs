use core::ptr::NonNull;

#[derive(Debug)]
pub struct ProcessedImage {
    pub(crate) inner: NonNull<sys::libraw_processed_image_t>,
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        unsafe { sys::libraw_dcraw_clear_mem(self.inner.as_ptr()) }
    }
}

// impl Deref for ProcessedImage {
//     type Target = *mut sys::libraw_processed_image_t;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl DerefMut for ProcessedImage {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

impl ProcessedImage {
    pub fn raw(&self) -> &sys::libraw_processed_image_t {
        unsafe { self.inner.as_ref() }
    }
    pub fn as_slice_u8(&self) -> &[u8] {
        self.as_slice::<u8>()
    }
    pub fn as_slice_u16(&self) -> &[u16] {
        self.as_slice::<u16>()
    }

    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.as_ref().data.as_ptr() as *const T,
                self.inner.as_ref().data_size as usize / std::mem::size_of::<T>(),
            )
        }
    }
    pub fn width(&self) -> u32 {
        self.raw().width.into()
    }
    pub fn height(&self) -> u32 {
        self.raw().height.into()
    }
    pub fn type_(&self) -> ImageFormat {
        ImageFormat::from(self.raw().type_)
    }
    pub fn bits(&self) -> u16 {
        self.raw().bits
    }
    pub fn colors(&self) -> u16 {
        self.raw().colors
    }
    pub fn size(&self) -> usize {
        self.raw().data_size as usize
    }
}

/// The thumbnail types that might be embedded inside a raw file
#[non_exhaustive]
#[cfg_attr(all(windows, target_env = "msvc"), repr(i32))]
#[cfg_attr(all(windows, target_env = "gnu"), repr(u32))]
#[cfg_attr(unix, repr(u32))]
pub enum ThumbnailFormat {
    Unknown = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_UNKNOWN,
    Jpeg = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEG,
    Bitmap = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP,
    Bitmap16 = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP16,
    Layer = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_LAYER,
    Rollei = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_ROLLEI,
    H265 = sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_H265,
}

impl From<sys::LibRaw_thumbnail_formats> for ThumbnailFormat {
    fn from(tformat: sys::LibRaw_thumbnail_formats) -> Self {
        use ThumbnailFormat::*;
        match tformat {
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_UNKNOWN => Unknown,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEG => Jpeg,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP => Bitmap,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP16 => Bitmap16,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_LAYER => Layer,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_ROLLEI => Rollei,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_H265 => H265,
            _ => Unknown,
        }
    }
}

/// The format the raw file might be encoded in
#[non_exhaustive]
#[cfg_attr(all(windows, target_env = "msvc"), repr(i32))]
#[cfg_attr(all(windows, target_env = "gnu"), repr(u32))]
#[cfg_attr(unix, repr(u32))]
pub enum ImageFormat {
    Jpeg = sys::LibRaw_image_formats_LIBRAW_IMAGE_JPEG,
    Bitmap = sys::LibRaw_image_formats_LIBRAW_IMAGE_BITMAP,
}

impl From<sys::LibRaw_image_formats> for ImageFormat {
    fn from(format: sys::LibRaw_image_formats) -> Self {
        use ImageFormat::*;
        match format {
            sys::LibRaw_image_formats_LIBRAW_IMAGE_JPEG => Jpeg,
            sys::LibRaw_image_formats_LIBRAW_IMAGE_BITMAP => Bitmap,
            _ => unimplemented!("Please use the correct bindings for this version of libraw"),
        }
    }
}
