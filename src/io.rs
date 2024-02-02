pub mod bindings;

#[cfg(feature = "debug")]
pub trait MaybeDebug: core::fmt::Debug {}

#[cfg(feature = "debug")]
impl<T: core::fmt::Debug> MaybeDebug for T {}

#[cfg(not(feature = "debug"))]
pub trait MaybeDebug {}

#[cfg(not(feature = "debug"))]
impl<T> MaybeDebug for T {}


//
// ```c++
// class DllDef LibRaw_abstract_datastream
// {
// public:
//   LibRaw_abstract_datastream() { };
//   virtual ~LibRaw_abstract_datastream(void) { }
//   virtual int valid() = 0;
//   virtual int read(void *, size_t, size_t) = 0;
//   virtual int seek(INT64, int) = 0;
//   virtual INT64 tell() = 0;
//   virtual INT64 size() = 0;
//   virtual int get_char() = 0;
//   virtual char *gets(char *, int) = 0;
//   virtual int scanf_one(const char *, void *) = 0;
//   virtual int eof() = 0;
// #ifdef LIBRAW_OLD_VIDEO_SUPPORT
//   virtual void *make_jas_stream() = 0;
// #endif
//   virtual int jpeg_src(void *);
//   virtual void buffering_off() {}
//   /* reimplement in subclass to use parallel access in xtrans_load_raw() if
//    * OpenMP is not used */
//   virtual int lock() { return 1; } /* success */
//   virtual void unlock() {}
//   virtual const char *fname() { return NULL; };
// #ifdef LIBRAW_WIN32_UNICODEPATHS
//   virtual const wchar_t *wfname() { return NULL; };
// #endif
// };
// ```
pub trait LibrawDatastream: Read + Seek + Eof + MaybeDebug {
    /// # Safety
    ///
    /// This function will be called from ffi (c++)
    /// This function is unsafe because it dereferences `this` ( self )
    unsafe fn read(&mut self, buffer: *const libc::c_void, sz: usize, nmemb: usize) -> i32 {
        // assert!(!this.is_null());
        // let this = &mut *this;
        let to_read = sz * nmemb;
        if to_read < 1 {
            return 0;
        }
        if self
            .read_exact(core::slice::from_raw_parts_mut(
                buffer.cast::<u8>().cast_mut(),
                to_read,
            ))
            .is_err()
        {
            -1i32
        } else {
            to_read as i32
        }
    }
    /// # Safety
    ///
    ///
    unsafe fn seek(&mut self, offset: i64, whence: u32) -> i32 {
        match whence {
            sys::SEEK_SET => {
                std::io::Seek::seek(self, std::io::SeekFrom::Start(offset as u64)).ok()
            }
            sys::SEEK_CUR => std::io::Seek::seek(self, std::io::SeekFrom::Current(offset)).ok(),
            sys::SEEK_END => std::io::Seek::seek(self, std::io::SeekFrom::End(offset)).ok(),
            _ => return 0,
        }
        .expect("Failed to seek");
        return 0;
    }
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer.
    unsafe fn tell(&mut self) -> i64 {
        // assert!(!this.is_null());
        // let this = unsafe { &mut *this };
        self.stream_position().map(|f| f as i64).unwrap_or(-1)
    }
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer.
    unsafe fn eof_(&mut self) -> i32 {
        // assert!(!this.is_null());
        // let this = unsafe { &mut *this };
        Eof::eof(self).map(|f| f as i32).unwrap_or(0) - 1
    }

    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer.
    unsafe fn size(&mut self) -> i64 {
        // assert!(!this.is_null());
        // let this = unsafe { &mut *this };
        Eof::len(self).map(|f| f as i64).unwrap_or(-1)
    }

    /// # Safety
    ///
    /// Reads a char from the buffer and casts it as i32 and in case of error returns -1
    unsafe fn get_char(&mut self) -> libc::c_int {
        // assert!(!this.is_null());
        // let this = unsafe { &mut *this };
        let mut buf = [0u8];
        if self.read_exact(&mut buf).is_err() {
            -1
        } else {
            buf[0] as libc::c_int
        }
    }
}

impl<T: Read + Seek + Eof + MaybeDebug> LibrawDatastream for T {}

pub trait LibrawBufferedDatastream: LibrawDatastream + BufRead + MaybeDebug {
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw pointer and is called from ffi.
    /// This function is like fgets(3)
    /// The C++ function which wraps this
    /// `char *LibRaw_buffer_datastream::gets(char *s, int sz)`
    unsafe fn gets(&mut self, buffer: *mut libc::c_char, size: libc::c_int) -> *const libc::c_char {
        if size < 1 {
            return core::ptr::null();
        }
        // assert!(!this.is_null());
        // let this = unsafe { &mut *this };
        if self.eof().unwrap_or(true) {
            return core::ptr::null();
        }
        assert!(!buffer.is_null());
        let size = size.clamp(u16::MIN.into(), u16::MAX.into()) as usize;
        let buffer: &mut [u8] = core::slice::from_raw_parts_mut(buffer.cast(), size);
        let x = read_until(self, b'\n', buffer);
        if x.is_err() {
            return core::ptr::null();
        }
        // if this.read_exact(&mut buf).is_err() {
        //     return core::ptr::null();
        // }
        // todo!()

        // Why *const i8 instead of u8 ?
        buffer.as_ptr().cast()
    }
}

impl<T: LibrawDatastream + BufRead + MaybeDebug> LibrawBufferedDatastream for T {}

use core::ops::{Deref, DerefMut};
// pub trait Libraw
use std::{
    io::{BufRead, Read, Seek, Write},
    pin::Pin,
};

pub trait Eof: Seek {
    fn len(&mut self) -> ::std::io::Result<u64> {
        use ::std::io::SeekFrom;
        let current = self.stream_position()?;
        // stream_len is still in nightly
        let end = ::std::io::Seek::seek(self, SeekFrom::End(0))?;
        if current != end {
            self.seek(SeekFrom::Start(current))?;
        }
        Ok(end)
    }

    fn is_empty(&mut self) -> ::std::io::Result<bool> {
        Ok(self.len()? == 0)
    }

    fn eof(&mut self) -> ::std::io::Result<bool> {
        use ::std::io::SeekFrom;
        let current = self.stream_position()?;
        // stream_len is still in nightly
        let end = ::std::io::Seek::seek(self, SeekFrom::End(0))?;
        if current == end {
            Ok(true)
        } else {
            self.seek(SeekFrom::Start(current))?;
            Ok(false)
        }
    }
}

impl<T: Seek> Eof for T {}

/// Abstract Datastream
///
/// Using the rust version of the abstract datastream
#[repr(transparent)]
pub struct AbstractDatastream<T: Read + Seek + Sized> {
    inner: T,
}

impl<T: Read + Seek + Sized> AbstractDatastream<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Read + Seek + Sized> Deref for AbstractDatastream<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Read + Seek + Sized> DerefMut for AbstractDatastream<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// impl<T: Read + Seek + Sized> Read for AbstractDatastream<T> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.inner.read(buf)
//     }
// }

// impl<T: Read + Seek + Sized> Seek for AbstractDatastream<T> {
//     fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
//         self.inner.seek(pos)
//     }
// }

// Taken from rust's stdlib with some changes
fn read_until<R: BufRead + ?Sized>(
    r: &mut R,
    delim: u8,
    mut buf: &mut [u8],
) -> Result<usize, std::io::Error> {
    // Bytes read from the Read trait
    let mut read = 0;
    // Bytes written to the buffer
    let mut write = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            let capped = available.len().min(buf.len().saturating_sub(write));
            match memchr::memchr(delim, &available[..capped]) {
                Some(i) => {
                    buf.write_all(&available[..capped])?;
                    (true, i + 1)
                }
                None => {
                    buf.write_all(&available[..capped])?;
                    (false, capped)
                }
            }
        };
        r.consume(used);
        read += used;
        write += used;
        if done || used == 0 {
            return Ok(read);
        }
    }
}

// impl<T: LibrawBufferedDatastream> AbstractDatastream<T> {
#[repr(transparent)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct LibrawOpaqueDatastream<'a> {
    inner: Box<dyn LibrawBufferedDatastream + 'a>,
}

impl<'a> LibrawOpaqueDatastream<'a> {
    pub fn new(inner: impl LibrawBufferedDatastream + MaybeDebug + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lod_valid(this: *mut LibrawOpaqueDatastream) -> i32 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    if this.inner.is_empty().unwrap_or(true) {
        0
    } else {
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn lod_read(
    this: *mut LibrawOpaqueDatastream,
    buffer: *const libc::c_void,
    sz: usize,
    nmemb: usize,
) -> i32 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    LibrawDatastream::read(&mut this.inner.as_mut(), buffer, sz, nmemb)
}

#[no_mangle]
pub unsafe extern "C" fn lod_seek(
    this: *mut LibrawOpaqueDatastream,
    offset: i64,
    whence: u32,
) -> i32 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    let inner = core::ptr::addr_of!(this.inner);
    let mut inner = core::ptr::read(inner);
    dbg!(&inner);
    let ret = LibrawDatastream::seek(&mut inner, offset, whence);
    core::mem::forget(inner);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn lod_tell(this: *mut LibrawOpaqueDatastream) -> i64 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    LibrawDatastream::tell(&mut this.inner)
}

#[no_mangle]
pub unsafe extern "C" fn lod_eof(this: *mut LibrawOpaqueDatastream) -> i32 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    LibrawDatastream::eof_(&mut this.inner)
}

#[no_mangle]
pub unsafe extern "C" fn lod_size(this: *mut LibrawOpaqueDatastream) -> i64 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    LibrawDatastream::size(&mut this.inner)
}

#[no_mangle]
pub unsafe extern "C" fn lod_get_char(this: *mut LibrawOpaqueDatastream) -> libc::c_int {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    LibrawDatastream::get_char(&mut this.inner)
}

#[no_mangle]
pub unsafe extern "C" fn lod_gets(
    this: *mut LibrawOpaqueDatastream,
    buffer: *mut libc::c_char,
    size: libc::c_int,
) -> *mut libc::c_char {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    // WARNING:
    // Can't be sure why it needs a *mut pointer but I hope it doesn't write to it
    LibrawBufferedDatastream::gets(&mut this.inner, buffer, size) as *mut libc::c_char
}

#[no_mangle]
pub unsafe extern "C" fn lod_scanf_one(
    this: *mut LibrawOpaqueDatastream,
    fmt: *const libc::c_char,
    val: *mut libc::c_void,
) -> libc::c_int {
    assert!(!this.is_null());
    use core::ffi::*;
    let this = unsafe { &mut *this };
    let fmt = unsafe { CStr::from_ptr(fmt) };
    // let val = unsafe { &mut *(val as *mut u32) };
    // let mut buf = [0u8; 4];
    // todo!()
    match fmt.to_bytes() {
        b"%d" => {
            let val = unsafe { &mut *(val as *mut i32) };
            let mut buf = [0u8; 4];
            if this.inner.read_exact(&mut buf).is_err() {
                return -1;
            }
            *val = i32::from_ne_bytes(buf);
            1
        }
        b"%f" => {
            let val = unsafe { &mut *(val as *mut f32) };
            let mut buf = [0u8; 4];
            if this.inner.read_exact(&mut buf).is_err() {
                return -1;
            }
            *val = f32::from_ne_bytes(buf);
            1
        }
        b"%s" => unimplemented!("skipped for now"),
        _ => return -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn lod_drop(this: *mut LibrawOpaqueDatastream) {
    assert!(!this.is_null());
    let this = unsafe { this.read() };
    panic!("dropping");
    drop(this)
}
