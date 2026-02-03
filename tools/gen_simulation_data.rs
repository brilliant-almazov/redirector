use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: gen_simulation_data <input_txt> <output_bin>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_path).expect("Failed to create output file");

    let mut id: i64 = 1;
    let mut count: u32 = 0;

    // First pass: count entries
    let entries: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect();

    // Write header: number of entries (u32)
    let num_entries = entries.len() as u32;
    output_file
        .write_all(&num_entries.to_le_bytes())
        .expect("Failed to write header");

    // Write each entry: id (i64), hashid_len (u16), hashid, url_len (u16), url
    for (hashid, url) in entries {
        // ID (i64, little-endian)
        output_file
            .write_all(&id.to_le_bytes())
            .expect("Failed to write id");

        // HashID length and bytes
        let hashid_bytes = hashid.as_bytes();
        let hashid_len = hashid_bytes.len() as u16;
        output_file
            .write_all(&hashid_len.to_le_bytes())
            .expect("Failed to write hashid length");
        output_file
            .write_all(hashid_bytes)
            .expect("Failed to write hashid");

        // URL length and bytes
        let url_bytes = url.as_bytes();
        let url_len = url_bytes.len() as u16;
        output_file
            .write_all(&url_len.to_le_bytes())
            .expect("Failed to write url length");
        output_file
            .write_all(url_bytes)
            .expect("Failed to write url");

        id += 1;
        count += 1;
    }

    println!("Written {} entries to {}", count, output_path);
}
