use extendr_api::prelude::*;
use std::process::{Command, Stdio};

/// Exposes the `fqtk demux` functionality as a Rust function that can be called from R.
/// 
/// @param inputs A character vector of input FASTQ file paths.
/// @param max_mismatches An integer specifying the maximum number of mismatches allowed during demultiplexing.
/// @param read_structures A character vector specifying the read structures for parsing barcodes and sequences.
/// @param sample_metadata A string specifying the path to the CSV or TSV file containing sample metadata.
/// @param output A string specifying the output directory or file path for demultiplexed results.
/// 
/// @return A character string indicating success.
/// @export
#[extendr]
fn fqtk_demux(
    inputs: Vec<String>,              
    max_mismatches: usize,            
    read_structures: Vec<String>,      
    sample_metadata: String,           
    output: String                    
) -> Result<String> {

    let mut command = Command::new("fqtk");
    command.arg("demux");

    for input in inputs {
        command.arg("--inputs").arg(input);
    }

    for read_structure in read_structures {
        command.arg("--read-structures").arg(read_structure);
    }

    command.arg("--sample-metadata").arg(sample_metadata)
           .arg("--output").arg(output)
           .arg("--max-mismatches").arg(max_mismatches.to_string());

    let status = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| extendr_api::Error::Other(format!(
            "Failed to execute fqtk demux: {}",
            e
        )))?;

    if status.success() {
        Ok("Demux operation completed successfully.".to_string())
    } else {
        let exit_code = status.code().unwrap_or(1);
        Err(extendr_api::Error::Other(format!(
            "fqtk demux failed (exit code {})",
            exit_code
        )))
    }
}

extendr_module! {
    mod fqtkWrapper;
    fn fqtk_demux;
}
