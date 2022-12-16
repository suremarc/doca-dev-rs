use core::ffi::CStr;
use core::fmt::Display;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use crate::doca_sys::*;

use self::private::Sealed;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Error(doca_error_t);

pub type Result<T> = core::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error code {}: {:?}", self.0, unsafe {
            CStr::from_ptr(doca_get_error_string(self.0))
        })
    }
}

#[inline]
pub fn errcode_to_result(code: doca_error_t) -> Result<()> {
    match code {
        doca_error_DOCA_SUCCESS => Ok(()),
        e => Err(Error(e)),
    }
}

#[repr(transparent)]
pub struct DevInfo(NonNull<doca_devinfo>);

pub struct DevInfoList(&'static mut [DevInfo]);

impl DevInfoList {
    pub fn new() -> Result<Self> {
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

impl Deref for DevInfoList {
    type Target = [DevInfo];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for DevInfoList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Drop for DevInfoList {
    fn drop(&mut self) {
        if let Err(e) =
            errcode_to_result(unsafe { doca_devinfo_list_destroy(self.as_mut_ptr() as *mut _) })
        {
            // todo: DOCA_LOG
        }
    }
}

#[repr(transparent)]
pub struct DevInfoRep(NonNull<doca_devinfo_rep>);

pub struct DevInfoRepList(&'static mut [DevInfoRep]);

impl DevInfoRepList {
    pub fn new(dev: &mut Dev, filter: i32) -> Result<Self> {
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

impl Deref for DevInfoRepList {
    type Target = [DevInfoRep];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for DevInfoRepList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Drop for DevInfoRepList {
    fn drop(&mut self) {
        if let Err(e) =
            errcode_to_result(unsafe { doca_devinfo_rep_list_destroy(self.as_mut_ptr() as *mut _) })
        {
            // todo: DOCA_LOG
        }
    }
}

#[repr(transparent)]
pub struct Dev(NonNull<doca_dev>);

impl Dev {
    pub fn new(info: &mut DevInfo) -> Result<Self> {
        let mut dev = core::ptr::null_mut();
        errcode_to_result(unsafe { doca_dev_open(info.0.as_ptr(), &mut dev as *mut _) })?;

        Ok(Self(NonNull::new(dev).expect(
            "doca_dev should be non-null after successful doca_dev_open",
        )))
    }

    pub fn info(&self) -> Option<DevInfo> {
        NonNull::new(unsafe { doca_dev_as_devinfo(self.0.as_ptr()) }).map(DevInfo)
    }
}

impl Drop for Dev {
    fn drop(&mut self) {
        if let Err(e) = errcode_to_result(unsafe { doca_dev_close(self.0.as_ptr()) }) {
            // todo: DOCA_LOG
        }
    }
}

pub struct Data(NonNull<doca_data>);

pub trait State: Sealed {}

pub struct Active;
impl Sealed for Active {}
impl State for Active {}

pub struct Inactive;
impl Sealed for Inactive {}
impl State for Inactive {}

#[repr(transparent)]
struct MmapInner(NonNull<doca_mmap>);

impl Deref for MmapInner {
    type Target = NonNull<doca_mmap>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MmapInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for MmapInner {
    fn drop(&mut self) {
        if let Err(e) = errcode_to_result(unsafe { doca_mmap_destroy(self.0.as_ptr()) }) {
            // todo: DOCA_LOG
        }
    }
}

#[repr(transparent)]
pub struct Mmap<S: State>(MmapInner, core::marker::PhantomData<S>);

impl Mmap<Inactive> {
    pub fn new(user_data: &Data) -> Result<Self> {
        let mut mmap = core::ptr::null_mut();
        errcode_to_result(unsafe { doca_mmap_create(user_data.0.as_ptr(), &mut mmap as *mut _) })?;

        Ok(Mmap(
            MmapInner(
                NonNull::new(mmap)
                    .expect("doca_mmap should be non-null after successful doca_mmap_start"),
            ),
            core::marker::PhantomData,
        ))
    }

    pub fn start(self) -> Result<Mmap<Active>> {
        errcode_to_result(unsafe { doca_mmap_start(self.0.as_ptr()) })?;

        Ok(Mmap(self.0, core::marker::PhantomData))
    }

    // this API doesn't exist yet, despite being mentioned multiple times in the docs:
    // pub fn property_set(&mut self) -> Result<()> {
    //     errcode_to_result(unsafe { doca_mmap_property_set() })?;

    //     Ok(())
    // }
}

impl Mmap<Active> {
    pub fn stop(self) -> Result<Mmap<Inactive>> {
        errcode_to_result(unsafe { doca_mmap_stop(self.0.as_ptr()) })?;

        Ok(Mmap(self.0, core::marker::PhantomData))
    }
}

mod private {
    pub trait Sealed {}
}
