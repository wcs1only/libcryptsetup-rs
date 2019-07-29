use std::io;
use std::ptr;

use crate::err::CryptSetupErr;

use cryptsetup_sys::*;
pub use cryptsetup_sys::{crypt_set_confirm_callback,crypt_set_log_callback};

pub struct CryptDevice {
    ptr: *mut crypt_device,
}

impl CryptDevice {
    pub fn init(device_path: &str) -> Result<Self, CryptSetupErr> {
        let mut cdevice: *mut crypt_device = ptr::null_mut();
        let cstr = to_str_ptr!(device_path)?;
        errno!(unsafe { crypt_init(&mut cdevice as *mut *mut crypt_device, cstr) })?;
        Ok(CryptDevice { ptr: cdevice })
    }

    pub fn init_with_data_device(
        device_path: &str,
        data_device_path: &str,
    ) -> Result<Self, CryptSetupErr> {
        let mut cdevice: *mut crypt_device = ptr::null_mut();
        let device_path_cstr = to_str_ptr!(device_path)?;
        let data_device_path_cstr = to_str_ptr!(data_device_path)?;
        errno!(unsafe {
            crypt_init_data_device(
                &mut cdevice as *mut *mut crypt_device,
                device_path_cstr,
                data_device_path_cstr,
            )
        })?;
        Ok(CryptDevice { ptr: cdevice })
    }

    pub fn init_by_name_and_header(
        name: &str,
        header_device_path: &str,
    ) -> Result<Self, CryptSetupErr> {
        let mut cdevice: *mut crypt_device = ptr::null_mut();
        let name_cstr = to_str_ptr!(name)?;
        let header_device_path_cstr = to_str_ptr!(header_device_path)?;
        errno!(unsafe {
            crypt_init_by_name_and_header(
                &mut cdevice as *mut *mut crypt_device,
                name_cstr,
                header_device_path_cstr,
            )
        })?;
        Ok(CryptDevice { ptr: cdevice })
    }

    pub fn init_by_name(name: &str) -> Result<Self, CryptSetupErr> {
        let mut cdevice: *mut crypt_device = ptr::null_mut();
        let name_cstr = to_str_ptr!(name)?;
        errno!(unsafe { crypt_init_by_name(&mut cdevice as *mut *mut crypt_device, name_cstr) })?;
        Ok(CryptDevice { ptr: cdevice })
    }

    pub fn as_ptr(&mut self) -> *mut crypt_device {
        self.ptr
    }
}

impl Drop for CryptDevice {
    fn drop(&mut self) {
        unsafe { crypt_free(self.ptr) }
    }
}

#[cfg(test)]
mod test {}
