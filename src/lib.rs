use libbrlapi_sys as ffi;
use std::convert::TryInto;
use std::ffi::{CString, CStr};
use std::ptr;

pub struct BrlAPI {
    handle: *mut ffi::brlapi_handle_t
}

impl BrlAPI {
    pub fn new() -> Self {
        let handle = unsafe {
            libc::malloc(
                ffi::brlapi_getHandleSize().try_into().unwrap()
            ) as *mut ffi::brlapi_handle_t
        };
        Self { handle }
    }
    pub fn open_connection(
        &self,
        auth: Option<&str>,
        host: Option<&str>
    ) -> (ffi::brlapi_fileDescriptor, String, String) {
        let auth = auth.map(|s| CString::new(s).unwrap());
        let host = host.map(|s| CString::new(s).unwrap());
        let auth = auth.map(|a| a.as_ptr()).unwrap_or(ptr::null());
        let host = host.map(|h| h.as_ptr()).unwrap_or(ptr::null());
        let mut settings = ffi::brlapi_connectionSettings_t { auth, host };
        let fd = unsafe {
            ffi::brlapi__openConnection(
                self.handle,
                &settings as *const ffi::brlapi_connectionSettings_t,
                &mut settings as *mut ffi::brlapi_connectionSettings_t
            )
        };
        let auth = unsafe { CStr::from_ptr(settings.auth) }.to_str().unwrap().to_string();
        let host = unsafe { CStr::from_ptr(settings.host) }.to_str().unwrap().to_string();
        (fd, auth, host)
    }
    pub fn close_connection(
        &self
    ) {
        unsafe { ffi::brlapi__closeConnection(self.handle) }
    }
    pub fn get_driver_name(&self) -> Result<String, std::str::Utf8Error> {
        let mut storage = [0u8; ffi::BRLAPI_MAXNAMELENGTH as usize + 1];
        let size = unsafe {
            ffi::brlapi__getDriverName(
                self.handle,
                storage.as_mut_ptr() as *mut ::std::os::raw::c_char,
                storage.len().try_into().unwrap()
            )
        } as usize;
        let str = std::str::from_utf8(&storage[..size - 1])?;
        Ok(str.to_owned())
    }
    pub fn get_model_identifier(&self) -> Result<String, std::str::Utf8Error> {
        let mut storage = [0u8; ffi::BRLAPI_MAXNAMELENGTH as usize + 1];
        let size = unsafe {
            ffi::brlapi__getModelIdentifier(
                self.handle,
                storage.as_mut_ptr() as *mut ::std::os::raw::c_char,
                storage.len().try_into().unwrap()
            ) as usize
        };
        let str = std::str::from_utf8(&storage[..size-1])?;
        Ok(str.to_owned())
    }
}

impl Drop for BrlAPI {
    fn drop(&mut self) {
        self.close_connection();
        unsafe { libc::free(self.handle as *mut core::ffi::c_void) }
    }
}

