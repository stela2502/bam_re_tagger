use rust_htslib::bam::{self, Read};
use std::path::Path;
use clap::Parser;
use std::fs;
use rayon::prelude::*;

fn process_bam(bam_file: &str, output_file: &str, from_tag: &str, to_tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input BAM file
    let bam_path = Path::new(bam_file);
    let mut bam_reader = bam::Reader::from_path(bam_path)?;

    // Create a new Header from the HeaderView
    let header = bam::Header::from_template(bam_reader.header());

    // Create output BAM file
    let mut bam_writer = bam::Writer::from_path(output_file, &header, bam::Format::Bam)?;

    const BUFFER_SIZE: usize = 1_000_000; // Define the buffer size (1 million reads)
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);

    // Read BAM records into the buffer
    for result in bam_reader.records() {
        let record = result?;
        buffer.push(record);

        // If we reach the buffer size, process the records
        if buffer.len() >= BUFFER_SIZE {
            // Process the buffer in parallel
            let processed_records: Vec<_> = buffer
                .par_iter() // Use par_iter to allow parallel processing
                .filter_map(|rec| {
                    let mut record = rec.clone(); // Clone to modify
                    if let Ok(value) = rec.aux(from_tag.as_bytes()) {
                        let _ = record.remove_aux(from_tag.as_bytes());
                        let _ = record.push_aux(to_tag.as_bytes(), value);
                        Some(record) // Return the modified record
                    } else {
                        None // Skip records without the tag
                    }
                })
                .collect();

            // Write the processed records in batches
            for record in processed_records {
                bam_writer.write(&record)?; // Write each modified record
            }

            // Clear the buffer for the next batch
            buffer.clear();
        }
    }

    // Process any remaining records in the buffer
    if !buffer.is_empty() {
        let processed_records: Vec<_> = buffer
            .par_iter() // Process the remaining records in parallel
            .filter_map(|rec| {
                let mut record = rec.clone(); // Clone to modify
                if let Ok(value) = rec.aux(from_tag.as_bytes()) {
                    let _ = record.remove_aux(from_tag.as_bytes());
                    let _ = record.push_aux(to_tag.as_bytes(), value);
                    Some(record)
                } else {
                    None
                }
            })
            .collect();

        // Write the processed records in batches
        for record in processed_records {
            bam_writer.write(&record)?; // Write each modified record
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// Path to the input BAM file
    #[clap(short, long)]
    input: String,

    /// Path to the output BAM file
    #[clap(short, long)]
    output: String,

    /// tag to search for in the BAM file (default: MA:Z:)
    #[clap(short = 'f', long, default_value = "MA:Z:")]
    from_tag: String,

    /// tag to replace the found tag with (default: UB:Z:)
    #[clap(short = 't', long, default_value = "UB:Z:")]
    to_tag: String,
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(&cli.output);
    if let Some(parent_dir) = path.parent() {
        match fs::create_dir_all(&parent_dir) {
            Ok(_) => (),
            Err(e) => panic!("I could not create the output path: {e}"),
        }
    }

    if let Err(err) = process_bam(&cli.input, &cli.output, &cli.from_tag, &cli.to_tag) {
        eprintln!("Error processing BAM file: {}", err);
    }

    println!("Finished");
}
