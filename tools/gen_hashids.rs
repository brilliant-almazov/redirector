//! Simple tool to generate hashids from numeric IDs
//! Run with: cargo run --release --bin gen_hashids < ids.txt > hashids.txt

use std::io::{self, BufRead, Write};

fn main() {
    let harsh = harsh::Harsh::builder()
        .salt("test-salt-for-load-testing")
        .length(6)
        .build()
        .expect("Failed to create harsh encoder");

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(id) = line.trim().parse::<u64>() {
                let hashid = harsh.encode(&[id]);
                writeln!(out, "{}", hashid).ok();
            }
        }
    }
}
