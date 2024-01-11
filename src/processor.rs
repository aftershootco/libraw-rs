use crate::*;
use core::ptr::NonNull;

pub struct Processor {
    pub(crate) inner: NonNull<sys::libraw_data_t>,
}
impl Processor {
    pub fn idata(&'_ self) -> &'_ sys::libraw_iparams_t {
        unsafe { &self.inner.as_ref().idata }
    }

    pub fn unpack(&mut self) -> Result<(), LibrawError> {
        let ret = unsafe { sys::libraw_unpack(self.inner.as_mut()) };
        LibrawError::check(ret)
    }
}
