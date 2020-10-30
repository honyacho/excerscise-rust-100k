use std::io;
use std::fs::File;
use std::io::BufReader;
use finalfusion::prelude::*;
use flate2::read::GzDecoder;

pub fn a67() -> std::io::Result<()> {

    let mut reader = BufReader::new(GzDecoder::new(BufReader::new(File::open("./similarity.bin.gz")?)));
    // let mut reader = BufReader::new(GzDecoder::new(BufReader::new(File::open("./GoogleNews-vectors-negative300.bin.gz")?)));
    let embeddings = Embeddings::read_word2vec_binary(&mut reader).unwrap();
    let embedding = embeddings.embedding("Berlin").unwrap();

    println!("{:?}", embedding);

    return Ok(());
}
