use std::io;

pub fn a15() -> std::io::Result<()> {
    let input = io::stdin();
    let mut buf = String::default();
    input.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut line_buf: Vec<String> = vec![String::from(""); n];
    let mut offset = 0;
    while input.read_line(&mut line_buf[offset % n])? > 0 {
        offset += 1;
    }
    for i in 0..n {
        let tmp = &line_buf[(offset + i) % n];
        println!("{}: {}", i, tmp.lines().last().unwrap());
    }
    return Ok(());
}
