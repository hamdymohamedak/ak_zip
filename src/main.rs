use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::io::Cursor;
use zip_extract::ZipExtractError;

fn main() -> io::Result<()> {
    println!("Enter the path to the zip file:");
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).expect("Failed to read input");

    println!("Enter the target path for extraction:");
    let mut target_path = String::new();
    io::stdin().read_line(&mut target_path).expect("Failed to read input");

    let input_value = input_value.trim(); // Trim newline characters
    let target_path = target_path.trim(); // Trim newline characters

    let file = File::open(input_value)?;
    let mut bytes = Vec::new();
    file.take(1024 * 1024 * 100) // Limit the file size to 100 MB for safety
        .read_to_end(&mut bytes)?;

    let target = PathBuf::from(target_path);

    let cursor = Cursor::new(bytes);

    match zip_extract::extract(cursor, &target, false) {
        Ok(_) => {
            println!("Extraction successful!");
            Ok(())
        },
        Err(err) => {
            match err {
                ZipExtractError::Io(io_err) => Err(io_err),
                other_err => Err(io::Error::new(io::ErrorKind::Other, format!("Zip extraction error: {:?}", other_err))),
            }
        }
    }
}
