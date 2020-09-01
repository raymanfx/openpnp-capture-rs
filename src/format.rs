use openpnp_capture_sys as ffi;

#[derive(Debug, Default, Copy, Clone)]
/// Capture format
pub struct Format {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixelformat
    pub fourcc: FourCC,
    /// Frames per second
    pub fps: u32,
    /// Bits per pixel
    pub bpp: u32,
}

impl Format {
    /// Builder: sets the width in pixels
    pub fn width(mut self, width: u32) -> Format {
        self.width = width;
        self
    }

    /// Builder: sets the height in pixels
    pub fn height(mut self, height: u32) -> Format {
        self.height = height;
        self
    }

    /// Builder: sets the pixelformat (buffers are automatically converted into RGB24)
    pub fn fourcc(mut self, fourcc: FourCC) -> Format {
        self.fourcc = fourcc;
        self
    }

    /// Builder: sets the frames per second
    pub fn fps(mut self, fps: u32) -> Format {
        self.fps = fps;
        self
    }
}

impl From<ffi::CapFormatInfo> for Format {
    fn from(val: ffi::CapFormatInfo) -> Self {
        Format {
            width: val.width,
            height: val.height,
            fourcc: FourCC::from(val.fourcc),
            fps: val.fps,
            bpp: val.bpp,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
/// Four character code representing a pixelformat
pub struct FourCC {
    pub repr: [u8; 4],
}

impl FourCC {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    /// Returns a pixelformat as four character code
    ///
    /// # Arguments
    ///
    /// * `repr` - Four characters as raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use openpnp_capture::format::FourCC;
    /// let fourcc = FourCC::new(b"YUYV");
    /// ```
    pub fn new(repr: &[u8; 4]) -> FourCC {
        FourCC { repr: *repr }
    }

    /// Returns the integer representation
    pub fn as_u32(&self) -> u32 {
        ((self.repr[3] as u32) << 24)
            | ((self.repr[2] as u32) << 16)
            | ((self.repr[1] as u32) << 8)
            | self.repr[0] as u32
    }
}

impl std::fmt::Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = std::str::from_utf8(&self.repr);
        if let Ok(string) = string {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

impl From<u32> for FourCC {
    fn from(code: u32) -> Self {
        let mut repr: [u8; 4] = [0; 4];
        repr[0] = (code & 0xff) as u8;
        repr[1] = ((code >> 8) & 0xff) as u8;
        repr[2] = ((code >> 16) & 0xff) as u8;
        repr[3] = ((code >> 24) & 0xff) as u8;
        FourCC { repr }
    }
}
