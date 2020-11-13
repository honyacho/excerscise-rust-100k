use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn a67() -> std::io::Result<()> {
    let file = File::open("./country_vectors.txt")?;
    let reader = BufReader::new(file);
    let mut itr = reader.lines();

    let mut name = itr.next();
    let mut vect = itr.next();
    while name.is_some() {
        let mut buf = vec![];
        for word in vect.unwrap()?.split_whitespace() {
            buf.push(word.parse::<f32>().unwrap());
        }
        name = itr.next();
        vect = itr.next();
    }

    return Ok(());
}
