use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

type Type = File;

pub fn get_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<Type>> {
    let file = File::open(path.as_ref()).expect("opens file");
    BufReader::new(file).lines()
}

pub fn get_buf_reader<P: AsRef<Path>>(path: P) -> BufReader<File> {
    let file = File::open(path.as_ref()).expect("opens file");
    BufReader::new(file)
}
