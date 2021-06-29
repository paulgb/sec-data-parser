use std::fs::{read_dir, File};
use std::io::BufReader;
use sec_data_parser::parse_submission;

fn main() {
    for file in read_dir("../data").unwrap() {
        let path = file.unwrap().path();
        println!("Reading: {:?}", &path);

        let mut reader = BufReader::new(File::open(path).unwrap());
        parse_submission(&mut reader).unwrap();
    }
}
