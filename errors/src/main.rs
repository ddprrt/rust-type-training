use std::{fs::File, io::Read, num::ParseIntError};

#[derive(Debug)]
enum NumberFromFileError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
}

impl std::fmt::Display for NumberFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberFromFileError::IoError(io) => {
                write!(f, "{io}")
            }
            NumberFromFileError::ParseError(parse) => {
                write!(f, "{parse}")
            }
        }
    }
}

impl From<ParseIntError> for NumberFromFileError {
    fn from(err: ParseIntError) -> Self {
        NumberFromFileError::ParseError(err)
    }
}

impl From<std::io::Error> for NumberFromFileError {
    fn from(err: std::io::Error) -> Self {
        NumberFromFileError::IoError(err)
    }
}

impl std::error::Error for NumberFromFileError {}

fn number_from_file(filename: Option<String>) -> Result<u64, NumberFromFileError> {
    let mut file_handle = File::open(filename.unwrap_or("data.txt".to_string()))?;

    let mut buffer = String::new();
    file_handle.read_to_string(&mut buffer)?;

    let num: u64 = buffer.parse()?;
    Ok(num)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = number_from_file(None)?;
    println!("{num}");
    Ok(())
}
