use bio::io::fastq;
use clap::Parser;
use std::fs::File;
use std::path::Path;

/// Parse one or more fastq files and print stats. Optionally output length filtered reads.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Minimum length of reads to consider
    #[arg(short, long, default_value_t = 0)]
    minlength: usize,

    /// Output file
    #[arg(short, long)]
    output: Option<String>,

    /// Input files, gzipped fastq
    #[arg(required = true)]
    input: Vec<String>,
}

fn main() {
    // Ingest command line arguments
    let args = Args::parse();

    // Builds on https://docs.rs/bio/0.32.0/bio/io/fastq/index.html

    // Open output file
    let mut writer: Option<fastq::Writer<File>> = None;
    if let Some(out_name) = args.output {
        let out_path = Path::new(&out_name);
        let out_file = File::create(out_path).unwrap();
        writer = Some(fastq::Writer::new(out_file));
    }

    // Iterate over input files
    for file_name in args.input {
        let reader = fastq::Reader::new(File::open(file_name).unwrap());
        // Iterate over records
        for record in reader.records() {
            let record = record.unwrap();
            let seq = record.seq();
            let id = record.id();
            // let desc = record.desc();
            let len = seq.len();
            if len >= args.minlength {
                println!("{} {}", id, len);
                if let Some(writer) = &mut writer {
                    writer
                        .write_record(&record)
                        .expect("Couldn't write output file");
                }
            }
        }
    }
}
