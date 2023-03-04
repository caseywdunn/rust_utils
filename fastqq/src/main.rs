use seq_io::fastq::{Reader, Record};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Create a FASTQ reader from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut reader = Reader::new(handle);

    // Iterate over each record and print the length and mean quality
    while let Some(result) = reader.next() {
        let record = result?;
        let length = record.seq().len();
        let mean_quality = record.qual().iter().map(|q| q.to_ascii_qual()).sum::<u32>() / length as u32;

        println!("Length: {}, Mean quality: {}", length, mean_quality);
    }

    Ok(())
}