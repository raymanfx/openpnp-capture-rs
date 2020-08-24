use openpnp_capture::{Device, Format};

fn main() {
    // Fetch some generic device information
    let devices = Device::enumerate();
    println!("Found {} devices.", devices.len());

    for index in devices {
        let dev = Device::new(index).expect("Failed to open device");
        // Fetch name and ID
        println!("[{}] {}", index, dev.name);
        println!("  ID = {}", dev.id);

        // Fetch the supported formats
        let formats = dev.formats();
        println!("  Formats = {}", formats.len());

        let formats: Vec<Format> = dev.formats();
        for format in &formats {
            println!(
                "  [{}] {}x{}@{}",
                format.fourcc, format.width, format.height, format.fps
            );
        }
    }
}
