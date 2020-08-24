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

        for i in 0..count {
            // Fetch name and ID
            let name = std::ffi::CStr::from_ptr(ffi::Cap_getDeviceName(context, i))
                .to_str()
                .unwrap();
            let id = std::ffi::CStr::from_ptr(ffi::Cap_getDeviceUniqueID(context, i))
                .to_str()
                .unwrap();
            println!("[{}] {}", i, name);
            println!("  ID = {}", id);

            // Fetch the supported formats
            let count = ffi::Cap_getNumFormats(context, i);
            println!("  Formats = {}", count);

            let mut formats: Vec<(FourCC, Vec<ffi::CapFormatInfo>)> = Vec::new();
            for fmt_index in 0..count as u32 {
                let mut format = ffi::CapFormatInfo {
                    width: 0,
                    height: 0,
                    fourcc: 0,
                    fps: 0,
                    bpp: 0,
                };
                let res = ffi::Cap_getFormatInfo(context, i, fmt_index, &mut format);
                match res {
                    ffi::CAPRESULT_OK => {
                        let fourcc = FourCC::from(format.fourcc);

                        let mut found = false;
                        for fmt in &mut formats {
                            if fmt.0 == fourcc {
                                found = true;
                                fmt.1.push(format);
                                break;
                            }
                        }

                        if !found {
                            formats.push((fourcc, vec![format]));
                        }
                    }
                    _ => {}
                }
            }

            for format in &formats {
                print!("    {} = [", format.0);
                for i in 0..format.1.len() {
                    let meta = &format.1[i];
                    print!("{}x{}@{}", meta.width, meta.height, meta.fps);
                    if i < format.1.len() - 1 {
                        print!(", ");
                    }
                }
                println!("]");
            }
        }

        // Release the library handle
        ffi::Cap_releaseContext(context);
    }
}
