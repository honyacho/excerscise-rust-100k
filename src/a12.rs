use std::io;
use std::fs::File;
use std::io::Write;

pub fn a12() -> std::io::Result<()> {
    let input = io::stdin();
    let mut buf = String::default();
    let mut file1 = File::create("col1.txt")?;
    let mut file2 = File::create("col2.txt")?;

    while input.read_line(&mut buf)? > 0 {
        let mut splitted = buf.split_whitespace();
        let c1 = String::from(splitted.next().unwrap_or(""));
        let c2 = String::from(splitted.next().unwrap_or(""));
        file1.write_all(c1.as_bytes())?;
        file1.write_all(b"\n")?;
        file2.write_all(c2.as_bytes())?;
        file2.write_all(b"\n")?;
    }
    return Ok(());
}
