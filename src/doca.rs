use core::ffi::CStr;
use core::fmt::Display;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use crate::doca_sys::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct DocaError(doca_error_t);

impl Display for DocaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error code {}: {:?}", self.0, unsafe {
            CStr::from_ptr(doca_get_error_string(self.0))
        })
    }
}

#[inline]
pub fn errcode_to_result(code: doca_error_t) -> Result<(), DocaError> {
    match code {
        doca_error_DOCA_SUCCESS => Ok(()),
        e => Err(DocaError(e)),
    }
}

#[repr(transparent)]
pub struct DocaDevInfo(NonNull<doca_devinfo>);

pub struct DocaDevInfoList(&'static mut [DocaDevInfo]);

impl DocaDevInfoList {
    pub fn new() -> Result<Self, DocaError> {
        let mut dev_list: *mut *mut doca_devinfo = core::ptr::null_mut();
        let mut nb_devs = 0u32;
        errcode_to_result(unsafe {
            doca_devinfo_list_create(&mut dev_list as *mut _, &mut nb_devs as *mut _)
        })?;

        Ok(Self(unsafe {
            core::slice::from_raw_parts_mut(dev_list as *mut _, nb_devs as usize)
        }))
    }
}

impl Deref for DocaDevInfoList {
    type Target = [DocaDevInfo];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DocaDevInfoList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for DocaDevInfoList {
    fn drop(&mut self) {
        if let Err(e) =
            errcode_to_result(unsafe { doca_devinfo_list_destroy(self.as_mut_ptr() as *mut _) })
        {
            // todo: DOCA_LOG
        }
    }
}

#[repr(transparent)]
pub struct DocaDevInfoRep(NonNull<doca_devinfo_rep>);

pub struct DocaDevInfoRepList(&'static mut [DocaDevInfoRep]);

impl DocaDevInfoRepList {
    pub fn new(dev: &mut DocaDev, filter: u32) -> Result<Self, DocaError> {
        let mut dev_list: *mut *mut doca_devinfo_rep = core::ptr::null_mut();
        let mut nb_devs = 0u32;
        errcode_to_result(unsafe {
            doca_devinfo_rep_list_create(
                dev.0.as_ptr(),
                filter,
                &mut dev_list as *mut _,
                &mut nb_devs as *mut _,
            )
        })?;

        Ok(Self(unsafe {
            core::slice::from_raw_parts_mut(dev_list as *mut _, nb_devs as usize)
        }))
    }
}

impl Deref for DocaDevInfoRepList {
    type Target = [DocaDevInfoRep];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DocaDevInfoRepList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for DocaDevInfoRepList {
    fn drop(&mut self) {
        if let Err(e) =
            errcode_to_result(unsafe { doca_devinfo_rep_list_destroy(self.as_mut_ptr() as *mut _) })
        {
            // todo: DOCA_LOG
        }
    }
}

#[repr(transparent)]
pub struct DocaDev(NonNull<doca_dev>);

impl DocaDev {
    pub fn new(info: &mut DocaDevInfo) -> Result<Self, DocaError> {
        let dev = core::ptr::null_mut();
        errcode_to_result(unsafe { doca_dev_open(info.0.as_ptr(), &mut dev as *mut _) })?;

        Ok(Self(NonNull::new(dev).expect(
            "doca_dev should be non-null after successful doca_dev_open",
        )))
    }

    fn info(&self) -> Option<DocaDevInfo> {
        NonNull::new(unsafe { doca_dev_as_devinfo(self.0.as_ptr()) }).map(DocaDevInfo)
    }
}

impl Drop for DocaDev {
    fn drop(&mut self) {
        if let Err(e) = errcode_to_result(unsafe { doca_dev_close(self.0.as_ptr()) }) {
            // todo: DOCA_LOG
        }
    }
}

// pub struct DocaBuf(*mut doca_buf);

// impl Clone for DocaBuf {
//     fn clone(&self) -> Self {

//     }
// }

// impl DocaBuf {
//     pub fn new() -> Self {
//         // doca_buf_g
//         todo!()
//     }
// }
