use lazy_static::lazy_static;
use openpnp_capture_sys as ffi;
use std::sync::Mutex;

#[derive(Debug)]
/// Library context
pub struct Context {
    pub inner: ffi::CapContext,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            inner: unsafe { ffi::Cap_createContext() },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::Cap_releaseContext(self.inner);
        }
    }
}

// Required by lazy_static
unsafe impl Send for Context {}

lazy_static! {
    pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::default());
}
