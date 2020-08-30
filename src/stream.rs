use openpnp_capture_sys as ffi;
use std::io;

use crate::context::CONTEXT;
use crate::device::Device;
use crate::format::Format;

#[derive(Debug)]
/// Capture device
pub struct Stream {
    /// Unique identifier
    id: i32,
    /// Format
    format: Format,
}

impl Stream {
    /// Returns a stream instance
    ///
    /// # Example
    ///
    /// ```
    /// use openpnp_capture::{Device, Format, Stream};
    /// let dev = Device::new(0);
    /// if let Some(dev) = &dev {
    ///     let format = Format::default().width(1280).height(720);
    ///     let stream = Stream::new(&dev, &format);
    ///     println!("Stream: {:?}", stream);
    /// }
    /// ```
    pub fn new(dev: &Device, format: &Format) -> Option<Self> {
        let context = CONTEXT.lock().unwrap().inner;

        // Look for the best format match
        let mut matched = (0, Format::default());
        let formats = dev.formats();
        for i in 0..formats.len() {
            // We assume width and height are always set and take into account FPS and FourCC if
            // requested.

            // Use L2 error metric for width, height and fourcc.
            // Prefer higher values for fps.

            let mut current = (matched.1.width as i64 - format.width as i64).pow(2)
                + (matched.1.height as i64 - format.height as i64).pow(2);
            if format.fps > 0 {
                current += (matched.1.fps as i64 - format.fps as i64).pow(2);
            }
            if format.fourcc.as_u32() > 0 {
                current +=
                    (matched.1.fourcc.as_u32() as i64 - format.fourcc.as_u32() as i64).pow(2);
            }

            let mut candidate = (formats[i].width as i64 - format.width as i64).pow(2)
                + (formats[i].height as i64 - format.height as i64).pow(2);
            if format.fps > 0 {
                // only penalize smaller fps values
                if formats[i].fps < format.fps {
                    candidate += (formats[i].fps as i64 - format.fps as i64).pow(2);
                }
            }
            if format.fourcc.as_u32() > 0 {
                candidate +=
                    (formats[i].fourcc.as_u32() as i64 - format.fourcc.as_u32() as i64).pow(2);
            }

            if candidate < current {
                matched = (i as u32, formats[i]);
            }
        }

        // fill in the bpp if necessary
        if matched.1.bpp == 0 {
            matched.1.bpp = matched.1.height * matched.1.width * 3 /* RGB24 */;
        }

        let id = unsafe { ffi::Cap_openStream(context, dev.index, matched.0) };
        match id {
            -1 => None,
            id => Some(Stream {
                id,
                format: matched.1,
            }),
        }
    }

    /// Returns the format in use
    pub fn format(&self) -> Format {
        self.format
    }

    /// Returns true when a new frame is available
    pub fn poll(&self) -> bool {
        let context = CONTEXT.lock().unwrap().inner;
        unsafe { ffi::Cap_hasNewFrame(context, self.id) == 1 }
    }

    /// Blocks until a new frame is available
    pub fn advance(&mut self) {
        while !self.poll() { /* busy loop */ }
    }

    /// Copy the current frame into a buffer
    pub fn read(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        let context = CONTEXT.lock().unwrap().inner;
        let frame_len = (self.format.height * self.format.width * 3/* RGB24 */) as usize;
        if buf.len() != frame_len {
            buf.resize(frame_len, 0);
        }

        // The buffer format is always RGB24
        let res = unsafe {
            ffi::Cap_captureFrame(
                context,
                self.id,
                buf.as_mut_ptr() as *mut std::ffi::c_void,
                buf.len() as u32,
            )
        };
        match res {
            ffi::CAPRESULT_OK => Ok(()),
            _ => Err(io::Error::new(io::ErrorKind::Other, "res != CAPRESULT_OK")),
        }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        let context = CONTEXT.lock().unwrap().inner;
        unsafe { ffi::Cap_closeStream(context, self.id) };
    }
}
