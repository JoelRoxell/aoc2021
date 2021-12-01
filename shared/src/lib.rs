use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let line_buff = io::BufReader::new(file).lines();

    let mut lines = vec![];

    for l in line_buff {
        lines.push(l.unwrap());
    }

    lines
}
