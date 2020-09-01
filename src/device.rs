use openpnp_capture_sys as ffi;

use crate::context::CONTEXT;
use crate::format;

#[derive(Debug)]
/// Capture device
pub struct Device {
    /// Index
    pub index: u32,
    /// Name
    pub name: String,
    /// Unique identifier
    pub id: String,
}

impl Device {
    /// Returns a list of valid device indices
    ///
    /// # Example
    ///
    /// ```
    /// use openpnp_capture::device::Device;
    /// let indices = Device::enumerate();
    /// println!("Devices: {:?}", indices);
    /// ```
    pub fn enumerate() -> Vec<u32> {
        let context = CONTEXT.lock().unwrap().inner;
        let count = unsafe { ffi::Cap_getDeviceCount(context) };

        let mut indices = Vec::new();
        for i in 0..count {
            indices.push(i);
        }

        indices
    }

    /// Returns a device instance
    ///
    /// # Example
    ///
    /// ```
    /// use openpnp_capture::device::Device;
    /// let dev = Device::new(0);
    /// println!("Device: {:?}", dev);
    /// ```
    pub fn new(index: u32) -> Option<Self> {
        let context = CONTEXT.lock().unwrap().inner;
        let name = unsafe { ffi::Cap_getDeviceName(context, index) };
        let id = unsafe { ffi::Cap_getDeviceUniqueID(context, index) };

        if name.is_null() || id.is_null() {
            return None;
        }

        Some(Device {
            index,
            name: unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap().to_string() },
            id: unsafe { std::ffi::CStr::from_ptr(id).to_str().unwrap().to_string() },
        })
    }

    /// Returns the supported formats
    ///
    /// # Example
    ///
    /// ```
    /// use openpnp_capture::device::Device;
    /// let dev = Device::new(0);
    /// if let Some(dev) = dev {
    ///     println!("Formats: {:?}", dev.formats());
    /// }
    /// ```
    pub fn formats(&self) -> Vec<format::Format> {
        let context = CONTEXT.lock().unwrap().inner;
        let count = unsafe { ffi::Cap_getNumFormats(context, self.index) };

        let mut formats = Vec::new();
        for i in 0..count as u32 {
            let mut format = ffi::CapFormatInfo {
                width: 0,
                height: 0,
                fourcc: 0,
                fps: 0,
                bpp: 0,
            };
            let res = unsafe { ffi::Cap_getFormatInfo(context, self.index, i, &mut format) };
            if let ffi::CAPRESULT_OK = res {
                formats.push(format::Format::from(format));
            }
        }

        formats
    }
}
