use dataparser_core::parser::core::DataParser;
use std::io;

fn main() -> io::Result<()> {
    let mut byte_data: Vec<u8> = vec![
        0, 0, 0, 123, 0, 0, 0, 13, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0,
        0, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 1, 2, 0, 0, 0, 1, 3,
    ];
    let mut data_parser = DataParser::new(&mut byte_data);
    let i32_example = data_parser.get_i32()?;
    let str_example = data_parser.get_string(false)?;
    let vector_example = data_parser.get_vector::<u8>()?;
    println!(
        "Decoded data: i32: {}, String: {}, Vector: {:?}",
        i32_example, str_example, vector_example
    );
    // Output:
    // Decoded data: i32: 123, String: Hello, world!, Vector: [1, 2, 3]
    Ok(())
}
