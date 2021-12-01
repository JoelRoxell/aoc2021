use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::str;

pub fn read_lines(filepath: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let line_buff = create_file_buffer(filepath).lines();
    let mut lines: Vec<String> = vec![];

    for l in line_buff {
        lines.push(l?);
    }

    Ok(lines)
}

pub fn read_symbol_separated_items(
    filepath: &str,
    sep: char,
) -> Result<Vec<String>, Box<dyn Error>> {
    let entires = create_file_buffer(filepath).split(sep as u8);
    let mut result_vec = vec![];

    for section in entires {
        let r = str::from_utf8(&section?)?.trim().to_string();

        result_vec.push(r);
    }

    Ok(result_vec)
}

pub fn create_file_buffer(filepath: &str) -> io::BufReader<File> {
    let f = File::open(filepath).unwrap_or_else(|_| panic!("couldn't find file {}", filepath));

    io::BufReader::new(f)
}
