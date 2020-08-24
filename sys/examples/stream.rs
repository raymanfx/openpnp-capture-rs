use openpnp_capture_sys as ffi;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
/// Four character code representing a pixelformat
pub struct FourCC {
    pub repr: [u8; 4],
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

impl From<[u8; 4]> for FourCC {
    fn from(val: [u8; 4]) -> Self {
        FourCC { repr: val }
    }
}

impl From<u32> for FourCC {
    fn from(code: u32) -> Self {
        let mut repr: [u8; 4] = [0; 4];
        repr[0] = (code & 0xff) as u8;
        repr[1] = ((code >> 8) & 0xff) as u8;
        repr[2] = ((code >> 16) & 0xff) as u8;
        repr[3] = ((code >> 24) & 0xff) as u8;
        FourCC::from(repr)
    }
}

fn main() {
    unsafe {
        // Initialize the library by creating a context
        let context = ffi::Cap_createContext();

        // Fetch some generic device information
        let count = ffi::Cap_getDeviceCount(context);
        println!("Found {} devices.", count);

        println!("Choosing first device (index = 0)");
        let index = 0;

        // Fetch name and ID
        let name = std::ffi::CStr::from_ptr(ffi::Cap_getDeviceName(context, index))
            .to_str()
            .unwrap();
        println!("[{}] {}", index, name);

        // Fetch the first available format
        let count = ffi::Cap_getNumFormats(context, index);
        if count <= 0 {
            panic!("No formats available");
        }

        println!("Choosing first format (index = 0)");

        let mut format = ffi::CapFormatInfo {
            width: 0,
            height: 0,
            fourcc: 0,
            fps: 0,
            bpp: 0,
        };

        let res = ffi::Cap_getFormatInfo(context, index, 0, &mut format);
        match res {
            ffi::CAPRESULT_OK => {}
            _ => {
                println!("Failed to read format");
            }
        }

        println!(
            "[0] {} ({}x{}@{})",
            FourCC::from(format.fourcc),
            format.width,
            format.height,
            format.fps
        );

        // Open a stream to capture frames
        let stream = ffi::Cap_openStream(context, index, 0);

        match ffi::Cap_isOpenStream(context, stream) {
            1 => {}
            _ => {
                println!("Failed to open stream");
            }
        }

        // Capture some frames
        let mut rgb_buffer =
            Vec::with_capacity((format.height * format.width * 3/* bpp */) as usize);

        let start = std::time::Instant::now();
        for _i in 0..10 {
            loop {
                // Wait for a new frame to arrive
                if ffi::Cap_hasNewFrame(context, stream) == 1 {
                    break;
                }
            }

            // Copy the frame into our buffer
            let res = ffi::Cap_captureFrame(
                context,
                stream,
                rgb_buffer.as_mut_ptr(),
                rgb_buffer.len() as u32,
            );
            match res {
                ffi::CAPRESULT_OK => {}
                _ => {
                    println!("Failed to capture frame");
                }
            }
        }
        println!(
            "{} FPS ({} MB/s)",
            1000 / start.elapsed().as_millis(),
            ((rgb_buffer.len() * 10) as u128 / start.elapsed().as_millis()) as f64 / 1000.0
        );

        // Close the stream once we're done
        match ffi::Cap_closeStream(context, stream) {
            ffi::CAPRESULT_OK => {}
            _ => {
                println!("Failed to close stream");
            }
        }

        // Release the library handle
        ffi::Cap_releaseContext(context);
    }
}
