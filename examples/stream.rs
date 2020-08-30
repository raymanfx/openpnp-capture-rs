use openpnp_capture::{Device, Format, Stream};

fn main() {
    // Fetch some generic device information
    let devices = Device::enumerate();
    println!("Found {} devices.", devices.len());

    println!("Choosing first device (index = 0)");
    let dev = Device::new(devices[0]).expect("Failed to open device");

    // Fetch name and ID
    println!("[{}] {}", dev.index, dev.name);

    // Create the stream
    let format = Format::default().width(1280).height(720).fps(30);
    let mut stream = Stream::new(&dev, &format).expect("Failed to create stream");

    println!(
        "[0] {} ({}x{}@{})",
        stream.format().fourcc,
        stream.format().width,
        stream.format().height,
        stream.format().fps
    );

    // Capture some frames
    let mut rgb_buffer = Vec::new();

    // Warmup
    stream.advance();

    let count = 10;
    let start = std::time::Instant::now();
    for _i in 0..count {
        stream.advance();
        stream
            .read(&mut rgb_buffer)
            .expect("Failed to capture frame");
    }
    println!(
        "{} FPS ({} MB/s)",
        1000 / (start.elapsed().as_millis() / count),
        ((rgb_buffer.len() * 10) as u128 / start.elapsed().as_millis()) as f64 / 1000.0
    );
}
