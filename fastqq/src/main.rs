use seq_io::fastq::{Reader, Record};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Create a FASTQ reader from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut reader = Reader::new(handle);

    // Iterate over each record and print the length and mean quality
    while let Some(result) = reader.next() {
        let record = result.unwrap();
        let length = record.seq().len();
        //let mean_quality = record.qual().iter().sum::<u32>() / length as u32;

        // Get the mean quality of the record
        let mean_quality:f64 = record.qual().iter().map(|q| q - 33).map(|q| q as f64).sum::<f64>() / length as f64;

        println!("{} {}", length, mean_quality);
    }

    Ok(())
}