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

        // https://docs.rs/bam/latest/bam/record/struct.Record.html
        // https://docs.rs/bam/latest/bam/record/tags/struct.TagViewer.html
        // https://docs.rs/bam/latest/bam/record/tags/enum.TagValue.html
        let tags = record.tags();
        let np = tags.get(b"np").unwrap();
        let mut np_val:i64 = 0;
        match np {
            bam::record::tags::TagValue::Int(val, _) => np_val= val,
            _ => panic!("Not an integer value"),
        }

        let q = record.qualities();

        // create geometric mean of qualities
        let mut q_sum:f64 = 0.0;
        for i in 0..q.len() {
            q_sum += 10.0_f64.powf(q[i] as f64 / -10.0);
        }
        let q_mean = 10.0 * q_sum.log10() / q.len() as f64;


        println!("{} {} {} {}", record.name(), record.sequence().len(), q_mean, np_val);

        // Print the record information
        //println!("{}:{}-{} {}", record.tid(), record.pos(), record.cigar().end_pos(), record.seq().len());
    }
}

