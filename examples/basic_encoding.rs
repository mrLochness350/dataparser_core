use dataparser_core::encoder::core::DataEncoder;
use std::io;

fn main() -> io::Result<()> {
    let a: &str = "Hello, world!";
    let b: i32 = 123;
    let c: Vec<u8> = vec![0x01, 0x02, 0x03];
    let mut data_encoder = DataEncoder::new();
    data_encoder.add_i32(b)?;
    data_encoder.add_string(a)?;
    data_encoder.add_slice(&c)?;
    let encoded_data = data_encoder.get_data()?;
    println!("Encoded data: {:?}", encoded_data);
    // Output:
    // Encoded data: [0, 0, 0, 123, 0, 0, 0, 13, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0, 0, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 1, 2, 0, 0, 0, 1, 3]
    Ok(())
}
