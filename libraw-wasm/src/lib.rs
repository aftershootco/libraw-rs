extern crate alloc;
extern crate libraw_sys as sys;

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ptr::NonNull;
use core::slice;
use core::sync::atomic::AtomicBool;

type Result<T> = core::result::Result<T, i32>;

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

#[derive(Debug)]
pub struct ProcessedImage {
    inner: NonNull<sys::libraw_processed_image_t>,
}

impl ProcessedImage {
    pub fn raw(&self) -> &sys::libraw_processed_image_t {
        unsafe { self.inner.as_ref() }
    }

    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.as_ref().data.as_ptr() as *const T,
                self.inner.as_ref().data_size as usize / std::mem::size_of::<T>(),
            )
        }
    }
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        unsafe { sys::libraw_dcraw_clear_mem(self.inner.as_ptr()) }
    }
}

pub struct Processor {
    inner: NonNull<sys::libraw_data_t>,
    dropped: Arc<AtomicBool>,
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

#[non_exhaustive]
#[cfg_attr(all(windows, target_env = "msvc"), repr(i32))]
#[cfg_attr(all(windows, target_env = "gnu"), repr(u32))]
#[cfg_attr(unix, repr(u32))]
pub enum LibrawConstructorFlags {
    None = sys::LibRaw_constructor_flags_LIBRAW_OPTIONS_NONE,
    // Depending on the version of libraw this is not generated
    NoMemErrCallBack = 1,
    // On some versions of libraw this is misspelled opions
    NoDataErrCallBack = sys::LibRaw_constructor_flags_LIBRAW_OPTIONS_NO_DATAERR_CALLBACK,
}

impl From<sys::LibRaw_thumbnail_formats> for ThumbnailFormat {
    fn from(tformat: sys::LibRaw_thumbnail_formats) -> Self {
        match tformat {
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_UNKNOWN => ThumbnailFormat::Unknown,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_JPEG => ThumbnailFormat::Jpeg,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP => ThumbnailFormat::Bitmap,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_BITMAP16 => ThumbnailFormat::Bitmap16,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_LAYER => ThumbnailFormat::Layer,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_ROLLEI => ThumbnailFormat::Rollei,
            sys::LibRaw_thumbnail_formats_LIBRAW_THUMBNAIL_H265 => ThumbnailFormat::H265,
            _ => ThumbnailFormat::Unknown,
        }
    }
}

/// You can pass the Processor to another thread since it doesn't use any thread_local values
impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            sys::libraw_free_image(self.inner.as_ptr());
            sys::libraw_close(self.inner.as_ptr());
        }
        self.dropped
            .store(true, core::sync::atomic::Ordering::SeqCst);
    }
}

impl Default for Processor {
    /// Returns libraw_init(0)
    fn default() -> Self {
        Self::new(LibrawConstructorFlags::None)
    }
}

unsafe impl Send for Processor {}
unsafe impl Sync for Processor {}

impl Processor {
    pub fn thumbs_list(&self) -> &sys::libraw_thumbnail_list_t {
        unsafe { &self.inner.as_ref().thumbs_list }
    }
    pub fn unpack_thumb_ex(&mut self, index: libc::c_int) -> Result<()> {
        InternalLibrawError::check(unsafe {
            sys::libraw_unpack_thumb_ex(self.inner.as_ptr(), index)
        })
        .expect("Cannot get thumbnails");
        Ok(())
    }

    /// Build Processor with options and params
    pub fn builder() -> ProcessorBuilder {
        ProcessorBuilder::default()
    }

    /// Calls libraw_init with the any of the constructor flags
    /// # May panic
    pub fn new(option: LibrawConstructorFlags) -> Self {
        let inner = unsafe { sys::libraw_init(option as u32) };
        assert!(!inner.is_null());
        Self {
            inner: NonNull::new(inner).expect("Failed to initialize libraw"),
            dropped: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn open_buffer(&mut self, buffer: impl AsRef<[u8]>) -> Result<()> {
        self.recycle()?;
        let buffer = buffer.as_ref();
        Ok(InternalLibrawError::check(unsafe {
            sys::libraw_open_buffer(
                self.inner.as_ptr(),
                buffer.as_ptr() as *const libc::c_void,
                buffer.len(),
            )
        })
        .expect("cannot open buffer."))
    }

    pub fn recycle(&mut self) -> Result<()> {
        unsafe { sys::libraw_recycle(self.inner.as_ptr()) };
        Ok(())
    }

    pub fn unpack(&mut self) -> Result<()> {
        InternalLibrawError::check(unsafe { sys::libraw_unpack(self.inner.as_ptr()) })?;
        Ok(())
    }

    /// Unpack the thumbnail for the file
    pub fn unpack_thumb(&mut self) -> Result<()> {
        InternalLibrawError::check(unsafe { sys::libraw_unpack_thumb(self.inner.as_ptr()) })
            .expect("Cannot get thumbnails");
        Ok(())
    }

    pub fn sizes(&'_ self) -> &'_ sys::libraw_image_sizes_t {
        unsafe { &self.inner.as_ref().sizes }
    }

    pub fn thumbnail(&'_ self) -> &'_ sys::libraw_thumbnail_t {
        unsafe { &self.inner.as_ref().thumbnail }
    }

    pub fn dcraw_process(&mut self) -> Result<()> {
        InternalLibrawError::check(unsafe { sys::libraw_dcraw_process(self.inner.as_ptr()) })?;
        Ok(())
    }

    pub fn dcraw_process_make_mem_image(&mut self) -> Result<ProcessedImage> {
        let mut errc = 0;
        let data = unsafe { sys::libraw_dcraw_make_mem_image(self.inner.as_ptr(), &mut errc) };
        assert!(!data.is_null());
        InternalLibrawError::to_result(
            errc,
            ProcessedImage {
                inner: NonNull::new(data).expect("Not null"),
            },
        )
    }

    pub fn to_jpeg_no_rotation(&mut self, quality: u8) -> Result<Vec<u8>> {
        // Since this image is possibly has a flip

        // Now check if libraw_unpack has been called already
        // If it has been call inner.image shouldn't be null
        if unsafe { self.inner.as_ref().image.is_null() } {
            self.unpack()?;
        }
        self.dcraw_process()?;
        let _processed = self.dcraw_process_make_mem_image()?;
        let processed = _processed.raw();

        // let data = unsafe {
        //     std::slice::from_raw_parts(
        //         processed.data.as_ptr() as *const u8,
        //         processed.data_size as usize,
        //     )
        // };

        match ImageFormat::from(processed.type_) {
            ImageFormat::Bitmap => {
                let colortype = match processed.bits {
                    8 => image::ColorType::Rgb8,
                    16 => image::ColorType::Rgb16,
                    _ => return Err(2),
                };
                let mut jpeg = Vec::new();
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, quality)
                    .encode(
                        _processed.as_slice(),
                        processed.width as u32,
                        processed.height as u32,
                        colortype,
                    )
                    .map_err(|_| 1)?;
                Ok(jpeg)
            }
            ImageFormat::Jpeg => {
                // structure contain in-memory image of JPEG file. Only type, data_size and data fields are valid (and nonzero);
                let jpeg = _processed.as_slice().to_vec();
                Ok(jpeg)
            }
        }
    }

    pub fn get_jpeg(&mut self) -> Result<Vec<u8>> {
        // First check if unpack_thumb has already been called.
        // If yes then don't call it

        // Check if (*inner).thumbnail.thumb is null
        if unsafe { (*self.inner.as_ptr()).thumbnail.thumb.is_null() } {
            self.unpack_thumb().unwrap();
        }
        let flip = self.sizes().flip;
        let thumbnail = self.thumbnail();
        let thumbnail_data = unsafe {
            slice::from_raw_parts(thumbnail.thumb as *const u8, thumbnail.tlength as usize)
        };

        match ThumbnailFormat::from(thumbnail.tformat) {
            ThumbnailFormat::Jpeg => {
                // Since the buffer is already a jpeg buffer return it as-is
                //
                // Don't use a Vec since a Vec's internal memory representation is entirely dependent
                // on the allocator used which might(is) be different in c/c++/rust
                let jpeg = thumbnail_data.to_vec();
                //let jpeg = Orientation::from(Flip::from(flip)).add_to(jpeg)?;
                Ok(jpeg)
            }
            ThumbnailFormat::Bitmap => {
                // Since this is a bitmap we have to generate the thumbnail from the rgb data from
                // here
                let mut jpeg = Vec::new();
                image::codecs::jpeg::JpegEncoder::new(&mut jpeg)
                    .encode(
                        thumbnail_data,
                        thumbnail.twidth as u32,
                        thumbnail.theight as u32,
                        image::ColorType::Rgb8,
                    )
                    .map_err(|_| 1)?;
                //let jpeg = Orientation::from(Flip::from(flip)).add_to(jpeg)?;
                Ok(jpeg)
            }
            _ => Err(1),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InternalLibrawError {
    UnspecifiedError = sys::LibRaw_errors_LIBRAW_UNSPECIFIED_ERROR,
    FileUnsupported = sys::LibRaw_errors_LIBRAW_FILE_UNSUPPORTED,
    RequestForNonexistentImage = sys::LibRaw_errors_LIBRAW_REQUEST_FOR_NONEXISTENT_IMAGE,
    OutOfOrderCall = sys::LibRaw_errors_LIBRAW_OUT_OF_ORDER_CALL,
    NoThumbnail = sys::LibRaw_errors_LIBRAW_NO_THUMBNAIL,
    UnsupportedThumbnail = sys::LibRaw_errors_LIBRAW_UNSUPPORTED_THUMBNAIL,
    InputClosed = sys::LibRaw_errors_LIBRAW_INPUT_CLOSED,
    NotImplemented = sys::LibRaw_errors_LIBRAW_NOT_IMPLEMENTED,
    UnsufficientMemory = sys::LibRaw_errors_LIBRAW_UNSUFFICIENT_MEMORY,
    DataError = sys::LibRaw_errors_LIBRAW_DATA_ERROR,
    IoError = sys::LibRaw_errors_LIBRAW_IO_ERROR,
    CancelledByCallback = sys::LibRaw_errors_LIBRAW_CANCELLED_BY_CALLBACK,
    BadCrop = sys::LibRaw_errors_LIBRAW_BAD_CROP,
    TooBig = sys::LibRaw_errors_LIBRAW_TOO_BIG,
    MempoolOverflow = sys::LibRaw_errors_LIBRAW_MEMPOOL_OVERFLOW,
}

impl From<i32> for InternalLibrawError {
    fn from(e: i32) -> Self {
        use InternalLibrawError::*;
        match e {
            // e if e == Success as i32 => Success,
            e if e == UnspecifiedError as i32 => UnspecifiedError,
            e if e == FileUnsupported as i32 => FileUnsupported,
            e if e == RequestForNonexistentImage as i32 => RequestForNonexistentImage,
            e if e == OutOfOrderCall as i32 => OutOfOrderCall,
            e if e == NoThumbnail as i32 => NoThumbnail,
            e if e == UnsupportedThumbnail as i32 => UnsupportedThumbnail,
            e if e == InputClosed as i32 => InputClosed,
            e if e == NotImplemented as i32 => NotImplemented,
            e if e == UnsufficientMemory as i32 => UnsufficientMemory,
            e if e == DataError as i32 => DataError,
            e if e == IoError as i32 => IoError,
            e if e == CancelledByCallback as i32 => CancelledByCallback,
            e if e == BadCrop as i32 => BadCrop,
            e if e == TooBig as i32 => TooBig,
            e if e == MempoolOverflow as i32 => MempoolOverflow,
            e if e == Self::SUCCESS => panic!("This call was a success"),
            _ => unreachable!("If the error is reached then libraw has added new error types"),
        }
    }
}

impl InternalLibrawError {
    pub const SUCCESS: i32 = sys::LibRaw_errors_LIBRAW_SUCCESS;
    pub fn check(code: i32) -> Result<()> {
        if code == Self::SUCCESS {
            Ok(())
        } else {
            Err(code)
        }
    }

    pub fn to_result<T>(code: i32, data: T) -> Result<T> {
        if code == Self::SUCCESS {
            Ok(data)
        } else {
            Err(code)
        }
    }
}

pub struct ProcessorBuilder {
    inner: NonNull<sys::libraw_data_t>,
}

impl ProcessorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Processor {
        Processor {
            inner: self.inner,
            dropped: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn with_params<P: IntoIterator<Item = Params>>(mut self, params: P) -> Self {
        let libraw_params = unsafe { &mut self.inner.as_mut().params };
        for param in params {
            match param {
                Params::Greybox(v) => libraw_params.greybox = v,
                Params::Cropbox(v) => libraw_params.cropbox = v,
                Params::Aber(v) => libraw_params.aber = v,
                Params::Gamm(v) => libraw_params.gamm = v,
                Params::UserMul(v) => libraw_params.user_mul = v,
                Params::Bright(v) => libraw_params.bright = v,
                Params::Threshold(v) => libraw_params.threshold = v,
                Params::HalfSize(v) => libraw_params.half_size = v as i32,
                Params::FourColorRgb(v) => libraw_params.four_color_rgb = v,
                Params::Highlight(v) => libraw_params.highlight = v,
                Params::UseAutoWb(v) => libraw_params.use_auto_wb = v as i32,
                Params::UseCameraWb(v) => libraw_params.use_camera_wb = v as i32,
                Params::UseCameraMatrix(v) => libraw_params.use_camera_matrix = v as i32,
                Params::OutputColor(v) => libraw_params.output_color = v,
                Params::OutputBps(v) => libraw_params.output_bps = v,
                Params::OutputTiff(v) => libraw_params.output_tiff = v,
                Params::OutputFlags(v) => libraw_params.output_flags = v,
                Params::UserFlip(v) => libraw_params.user_flip = v,
                Params::UserQual(v) => libraw_params.user_qual = v,
                Params::UserBlack(v) => libraw_params.user_black = v,
                Params::UserCblack(v) => libraw_params.user_cblack = v,
                Params::UserSat(v) => libraw_params.user_sat = v,
                Params::MedPasses(v) => libraw_params.med_passes = v,
                Params::AutoBrightThr(v) => libraw_params.auto_bright_thr = v,
                Params::AdjustMaximumThr(v) => libraw_params.adjust_maximum_thr = v,
                Params::NoAutoBright(v) => libraw_params.no_auto_bright = v,
                Params::UseFujiRrotate(v) => libraw_params.use_fuji_rotate = v,
                Params::GreenMatching(v) => libraw_params.green_matching = v,
                Params::DcbIterations(v) => libraw_params.dcb_iterations = v,
                Params::DcbEnhanceFl(v) => libraw_params.dcb_enhance_fl = v,
                Params::FbddNoiserd(v) => libraw_params.fbdd_noiserd = v,
                Params::ExpCorrec(v) => libraw_params.exp_correc = v,
                Params::ExpShift(v) => libraw_params.exp_shift = v,
                Params::ExpPreser(v) => libraw_params.exp_preser = v,
                Params::NoAutoScale(v) => libraw_params.no_auto_scale = v,
                Params::NoInterpolation(v) => libraw_params.no_interpolation = v,
            }
        }
        self
    }
}
impl Default for ProcessorBuilder {
    fn default() -> Self {
        let inner = unsafe { sys::libraw_init(LibrawConstructorFlags::None as u32) };
        assert!(!inner.is_null());
        Self {
            inner: NonNull::new(inner).expect("non null"),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Params {
    Greybox([u32; 4]),
    Cropbox([u32; 4]),
    Aber([f64; 4]),
    Gamm([f64; 6]),
    UserMul([f32; 4usize]),
    Bright(f32),
    Threshold(f32),
    HalfSize(bool),
    FourColorRgb(i32),
    Highlight(i32),
    UseAutoWb(bool),
    UseCameraWb(bool),
    UseCameraMatrix(bool),
    OutputColor(i32),
    // OutputProfile: *mut libc::c_char,
    // CameraProfile: *mut libc::c_char,
    // BadPixels: *mut libc::c_char,
    // DarkFrame: *mut libc::c_char,
    OutputBps(i32),
    OutputTiff(i32),
    OutputFlags(i32),
    UserFlip(i32),
    UserQual(i32),
    UserBlack(i32),
    UserCblack([i32; 4usize]),
    UserSat(i32),
    MedPasses(i32),
    AutoBrightThr(f32),
    AdjustMaximumThr(f32),
    NoAutoBright(i32),
    UseFujiRrotate(i32),
    GreenMatching(i32),
    DcbIterations(i32),
    DcbEnhanceFl(i32),
    FbddNoiserd(i32),
    ExpCorrec(i32),
    ExpShift(f32),
    ExpPreser(f32),
    NoAutoScale(i32),
    NoInterpolation(i32),
}
