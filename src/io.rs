pub mod bindings;
///  Input layer abstraction
///
///  class LibRaw_abstract_datastream - abstract RAW read interface
///
///  LibRaw reads RAW-data by calling (virtual) methods of C++ object derived from LibRaw_abstract_datastream. This C++ class does not implement any read, but defines interface to be called. Call to base class methods always results in error.
///
///  LibRaw_abstract_datastream class methods
///
///  Object verification
///
///  virtual int valid()
///      Checks input datastream validity. Returns 1 on valid stream and 0 if datastream was created on non-valid input parameters (wrong filename for file stream and so on).
///
///  Stream read and positioning
///
///  This group of methods implements file object (FILE*) semantics.
///
///  - `virtual int read(void * ptr,size_t size, size_t nmemb)`
///      Similar to fread(ptr,size,nmemb,file).
///  - `virtual int seek(off_t o, int whence)`
///      Similar to fseek(file,o,whence).
///  - `virtual int tell(`
///      Similar to ftell(file).
///  - `virtual int get_char()`
///      Similar to getc(file)/fgetc(file).
///  - `virtual char* gets(char *s, int n)`
///      Similar to fgets(s,n,file).
///  - `virtual int eof()`
///      Similar to feof(file).
///  - `virtual int scanf_one(const char *fmt, void *val)`
///      Simplified variant of fscanf(file,fmt,val): format string is always contains one argument to read. So, variable args call is not needed and only one pointer to data passed.
///  - `virtual int jpeg_src(void * p);`
///      Initializes read structures in j_decompress_ptr object passed as *p. This object is used by libjpeg for JPEG data reading from datastream.
///
///      Returns -1 on error and 0 on success.
///  - `virtual void * make_jas_stream();`
///      Creates LibJasper input stream (for JPEG2000 decoding).
///
///      returns NULL on error or data pointer on success.
///
///  Other methods
///
///  This group of methods includes several supplementary calls. These calls are used to temporary switch to another data stream (file and/or memory buffer).
///
///  virtual const char* fname()
///      Returns name of opened file if datastream object knows it (for example, LibRaw_file_datastream used). Filename used in:
///
///          error notification callbacks;
///          generation of filename of JPEG-file with metadata when needed (i.e. cameras with 'Diag RAW hack').
///
///  virtual int subfile_open(const char *fn)
///      This call temporary switches input to file fn. Returns 0 on success and error code on error.
///      The function used to read metadata from external JPEG file (on cameras with "Diag RAW hack").
///      This call is not implemented for LibRaw_buffer_datastream, so external JPEG processing is not possible when buffer datastream used.
///      This function should be implemented in real input class, base class call always return error.
///      Working implementation sample can be found in LibRaw_file_datastream implementation in libraw/libraw_datastream.h file.
///  virtual void subfile_close()
///      This call switches input stream from temporary open file back to main data stream.
///  virtual int tempbuffer_open(void *buf, size_t size)
///      This call temporary switches input to LibRaw_buffer_datastream object, created from buf.
///      This method is needed for Sony encrypted metadata parser.
///
///      This call implemented in base class (LibRaw_abstract_datastream), there is no need to reimplement in in derived classes.
///      Possible activity of temporary datastream requires very accurate programming when implementing datastreams derived from base LibRaw_abstract_datastream. See below for more details.
///  virtual void tempbuffer_close()
///      This call switch input back from temporary datastream to main stream. This call implemented in base LibRaw_abstract_datastream class.
///
///  Derived input classes included in LibRaw
///
///  There is three "standard" input classes in LibRaw distribution:
///
///      LibRaw_file_datastream implements input from file (in filesystem).
///      LibRaw_bigfile_datastream slower I/O, but files larger than 2Gb are supported.
///      LibRaw_buffer_datastream implements input from memory buffer.
///
///  LibRaw C++ interface users can implement their own input classes and use them via LibRaw::open_datastream call. Requirements and implementation specifics are described below.
///

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
pub trait LibrawDatastream: Read + Seek + Eof {
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

impl<T: Read + Seek + Eof> LibrawDatastream for T {}

pub trait LibrawBufferedDatastream: LibrawDatastream + BufRead {
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

impl<T: LibrawDatastream + BufRead> LibrawBufferedDatastream for T {}

use core::ops::{Deref, DerefMut};
// pub trait Libraw
use std::io::{BufRead, Read, Seek, Write};

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
#[repr(C)]
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
#[repr(C)]
pub struct LibrawOpaqueDatastream<'a> {
    inner: Box<dyn LibrawBufferedDatastream + 'a>,
}

impl<'a> LibrawOpaqueDatastream<'a> {
    pub fn new(inner: impl LibrawBufferedDatastream + 'a) -> Self {
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
    LibrawDatastream::read(&mut this.inner, buffer, sz, nmemb)
}

#[no_mangle]
pub unsafe extern "C" fn lod_seek(
    this: *mut LibrawOpaqueDatastream,
    offset: i64,
    whence: u32,
) -> i32 {
    assert!(!this.is_null());
    let this = unsafe { &mut *this };
    LibrawDatastream::seek(&mut this.inner, offset, whence)
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
    todo!();
}
