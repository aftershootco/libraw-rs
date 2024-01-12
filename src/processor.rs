use crate::*;
use core::ptr::NonNull;

pub struct Processor<'reader> {
    pub(crate) inner: NonNull<sys::libraw_data_t>,
    pub(crate) _marker: std::marker::PhantomData<&'reader ()>,
}

impl<'reader> Processor<'reader> {
    pub unsafe fn new(inner: NonNull<sys::libraw_data_t>) -> Self {
        Self {
            inner,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn idata(&'_ self) -> &'_ sys::libraw_iparams_t {
        unsafe { &self.inner.as_ref().idata }
    }

    pub fn unpack(&mut self) -> Result<(), LibrawError> {
        let ret = unsafe { sys::libraw_unpack(self.inner.as_mut()) };
        LibrawError::check(ret)
    }
}
