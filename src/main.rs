use rust_htslib::bam::{self, Read};
use std::path::Path;
use clap::Parser;
use std::fs;

fn process_bam(bam_file: &str, output_file: &str, from_tag: &str, to_tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input BAM file
    let bam_path = Path::new(bam_file);
    let mut bam_reader = bam::Reader::from_path(bam_path)?;

    // Create a new Header from the HeaderView
    let header = bam::Header::from_template(bam_reader.header());

    // Create output BAM file
    let mut bam_writer = bam::Writer::from_path(output_file, &header, bam::Format::Bam)?;

    for result in bam_reader.records() {
        // Unwrap the result; handle errors if they occur
        let checks = result?;
        let mut record = checks.clone(); // Make a mutable copy of the record

        // Initialize a flag to indicate whether to write the record
        let mut write = false;

        // Check if the record has the 'from_tag' and get its value
        if let Ok(value) = checks.aux(from_tag.as_bytes()) {
            // Remove the old tag
            let _ = record.remove_aux(from_tag.as_bytes());

            // Add the new tag with the same value
            let _ = record.push_aux(to_tag.as_bytes(), value); // This method sets the new tag

            write = true; // Mark that we need to write the modified record
        }


        // Write the modified record to the output BAM file if necessary
        if write {
            bam_writer.write(&record)?; // Write the modified record
        }
        else {
            // Optionally, write the original (unmodified) record
            bam_writer.write(&record)?; // Write the original record unchanged
        }
    }

    // // Collect records from the BAM file
    // let records: Result<Vec<Record>, _> = bam_reader.records().collect();
    // // Process each record sequentially

    // match records {
    //     Ok(records) => {
    //         for result in records {
    //             // Here, result is of type Record, so we can clone it directly
    //             let mut record: Record = result.clone(); 
    //             let mut write = false;

    //             // Check if the record has the 'from_tag' and get its value
    //             if let Ok(value) = result.aux(from_tag.as_bytes()) {
    //                 // Remove the old tag
    //                 let _ =record.remove_aux(from_tag.as_bytes());

    //                 // Add the new tag with the same value
    //                 let _ =record.push_aux(to_tag.as_bytes(), value); // This method sets the new tag

    //                 write = true;
    //             }
    //             if write {
    //                 // Write the modified (or unmodified) record to the output BAM file
    //                 bam_writer.write(&record)?;
    //             }
    //         }
    //     },
    //     Err(_) => {},
    // }

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
        match fs::create_dir_all(&parent_dir){
            Ok(_) => (),
            Err(e) => panic!("I could not create the outpath: {e}")
        };
    }

    let _ = process_bam(&cli.input, &cli.output, &cli.from_tag, &cli.to_tag);

    println!("Finished");
}

