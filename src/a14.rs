use std::io;

pub fn a14() -> std::io::Result<()> {
    let input = io::stdin();
    let mut buf = String::default();
    input.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    for _ in 0..n {
        if input.read_line(&mut buf)? > 0 {
            print!("{}", buf);
            buf.clear();
        } else {
            break;
        }
    }

    return Ok(());
}
