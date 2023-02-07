use clap::Parser;
use std::fs::File;
use seq_io::fastq::{Reader,Record};


/// Parse one or more fastq files and print stats. Optionally output length filtered reads.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// k-mer length
    #[arg(short, long, default_value_t = 0)]
    minlength: usize,

    /// output file
    #[arg(short, long)]
    output: Option<String>, 

    /// Input files, gzipped fastq
    #[arg(required = true)]
    input: Vec<String>,
}

fn main() {
    // Ingest command line arguments
    let args = Args::parse();

    // Open input files
    let mut readers: Vec<Reader<std::fs::File>> = Vec::new();
    for file in args.input {
        let reader = Reader::from_file(file).unwrap();
        readers.push(reader);
    }

    // Open output file
    let mut out_file: Option<File> = None;
    if let Some(output) = args.output {
        out_file = File::create(output);
    }

    // Iterate over input files
    for reader in readers {
        // Iterate over records
        for record in reader.records() {
            let mut record = record.unwrap();
            let seq = record.seq();
            let id = record.id().unwrap();
            // let desc = record.desc();
            let len = seq.len();
            if len >= args.minlength {
                println!("{} {}", id, len);
                if let Some(writer) = &mut out_file {
                    record.write_to(&mut writer);
                }
            }
        }
    }

}
