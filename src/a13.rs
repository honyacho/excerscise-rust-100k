use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn a13() -> std::io::Result<()> {
    let mut buf1 = String::default();
    let mut buf2 = String::default();
    let mut file1 = BufReader::new(File::open("col1.txt")?);
    let mut file2 = BufReader::new(File::open("col2.txt")?);

    while file1.read_line(&mut buf1)? > 0 && file2.read_line(&mut buf2)? > 0 {
        println!("{}\t{}", buf1, buf2);
    }
    return Ok(());
}
