// Disclosure, Most of this code is AI generated
extern crate csv;
extern crate rust_tr;

use rust_tr::calculate_standard_deviation;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use sys_info::mem_info;

fn record_elapsed_time_to_file(file_name: &str, elapsed_time_seconds: f64) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    let mut file = std::io::BufWriter::new(file);

    writeln!(file, "Elapsed Time: {:.6} seconds\n\n", elapsed_time_seconds)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let csv_file_path = "datasets/cereal.csv";
    let csv_file = File::open(csv_file_path)?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(csv_file);

    let mut cereal_values: Vec<f64> = Vec::new();

    for record in csv_reader.records() { // Renamed 'result' to 'record'
        let column_name = record?; // Renamed 'result' to 'column_name'
        let value: f64 = column_name[2].parse()?; // Update column index
        cereal_values.push(value);
    }

    let start_time = Instant::now();
    // Calculate and print the standard deviation
    let standard_deviation = calculate_standard_deviation(&cereal_values);

    println!("Standard Deviation: {:.4}", standard_deviation);

    let end_time = Instant::now();

    let elapsed_time_seconds = end_time.duration_since(start_time).as_secs_f64();
    let memory_info = mem_info().unwrap();

    match record_elapsed_time_to_file("rust_time.md", elapsed_time_seconds) {
        Ok(_) => {}
        Err(e) => println!("Error: {:?}", e),
    }

    println!(
        "Memory Usage: {}%",
        (memory_info.total - memory_info.avail) as f32 / memory_info.total as f32 * 100.0
    );
    println!("Elapsed time: {:.6} seconds", elapsed_time_seconds);

    Ok(())
}
