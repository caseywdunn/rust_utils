//use bam::BamReader;
use std::env;

fn main() {
    // Get the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the BAM file
    //let mut bam_reader = bam::BamReader::from_path(filename).unwrap();
    let bam_reader = bam::BamReader::from_path(filename, 4).unwrap();

    // Loop through each record in the BAM file
    for record_result in bam_reader {
        // Unwrap the record result
        let record = record_result.unwrap();

        // println!("{}", record.seq.len);

        // Print the record information
        //println!("{}:{}-{} {}", record.tid(), record.pos(), record.cigar().end_pos(), record.seq().len());
    }
}

