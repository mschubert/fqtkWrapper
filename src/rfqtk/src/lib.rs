use extendr_api::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

struct Line {
    text: String,
}

fn spawn_reader<R: std::io::Read + Send + 'static>(reader: R, tx: mpsc::Sender<Line>) {
    thread::spawn(move || {
        let buf = BufReader::new(reader);
        for line in buf.lines().flatten() {
            let _ = tx.send(Line { text: line });
        }
    });
}

fn emit_message(line: &str) {
    let _ = extendr_api::call!("message", line);
}

/// Exposes the `fqtk demux` functionality as a Rust function that can be called from R.
/// 
/// @param inputs A character vector of input FASTQ file paths.
/// @param max_mismatches An integer specifying the maximum number of mismatches allowed during demultiplexing.
/// @param read_structures A character vector specifying the read structures for parsing barcodes and sequences.
/// @param sample_metadata A string specifying the path to the CSV or TSV file containing sample metadata.
/// @param output A string specifying the output directory or file path for demultiplexed results.
/// @param verbose A boolean indicating whether to relay stdout/stderr as R messages.
/// 
/// @return An integer exit code (0 on success, non-zero on failure).
#[extendr]
fn fqtk_demux_internal(
    inputs: Vec<String>,              
    max_mismatches: usize,            
    read_structures: Vec<String>,      
    sample_metadata: String,           
    output: String,
    verbose: bool
) -> i32 {

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

    let mut child = match command.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(_e) => return 1,
    };

    let (tx, rx) = mpsc::channel();
    if let Some(stdout) = child.stdout.take() {
        spawn_reader(stdout, tx.clone());
    }
    if let Some(stderr) = child.stderr.take() {
        spawn_reader(stderr, tx.clone());
    }
    drop(tx);

    while let Ok(line) = rx.recv() {
        if verbose && !line.text.is_empty() {
            emit_message(&line.text);
        }
    }

    match child.wait() {
        Ok(status) => status.code().unwrap_or(1),
        Err(_e) => 1,
    }
}

extendr_module! {
    mod fqtkWrapper;
    fn fqtk_demux_internal;
}
