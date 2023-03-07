//use bam::BamReader;
use std::env;

fn main() {
    // Get the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the BAM file
    //let mut bam_reader = bam::BamReader::from_path(filename).unwrap();
    let bam_reader = bam::BamReader::from_path(filename, 4).unwrap();

    println!("name length np quality_mean readscore");

    // Loop through each record in the BAM file
    for record_result in bam_reader {
        // Unwrap the record result
        let record = record_result.unwrap();

        // https://docs.rs/bam/latest/bam/record/struct.Record.html
        // https://docs.rs/bam/latest/bam/record/tags/struct.TagViewer.html
        // https://docs.rs/bam/latest/bam/record/tags/enum.TagValue.html
        let tags = record.tags();

        let np = tags.get(b"np").unwrap();
        let np_val: i64 = match np {
            bam::record::tags::TagValue::Int(val, _) => val,
            _ => panic!("Not an integer value"),
        };

        let rq = tags.get(b"rq").unwrap();
        let rq_val: f32 = match rq {
            bam::record::tags::TagValue::Float(val) => val,
            _ => panic!("Not a float value"),
        };

        // create geometric mean of qualities
        let q = record.qualities().raw();
        let mut q_mean: f64 = 0.0;
        for i in 0..q.len() {
            q_mean += (q[i] as f64).ln();
        }
        q_mean /= q.len() as f64;
        q_mean = q_mean.exp();

        let name = std::str::from_utf8(record.name()).unwrap();

        println!(
            "{} {} {} {:.2} {}",
            name,
            record.sequence().len(),
            np_val,
            q_mean,
            rq_val
        );
    }
}
