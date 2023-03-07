

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <input.bam> <n> <output.bam>", args[0]);
        std::process::exit(1);
    }
    let input_bam = &args[1];
    let n = args[2].parse().unwrap();
    let output_bam = &args[3];

    // Open input BAM file
    let mut reader = bam::BamReader::from_path(input_bam).unwrap();

    // Open output BAM file
    let header = reader.header().clone();
    let mut writer = bam::BamWriter::build()
        .from_path(output_bam, &header)
        .unwrap();

    // Write n records to output BAM file
    let mut records_written = 0;
    for record in reader {
        let record = record.unwrap();
        writer.write(&record).unwrap();
        records_written += 1;
        if records_written == n {
            break;
        }
    }

}