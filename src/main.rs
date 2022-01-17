use std::io::{self, BufRead};
use std::env;
use std::process;
use std::collections::HashMap;

fn main() {
    // input argument is filter target (list)
    let mut hm = HashMap::new();
    for (idx, arg) in env::args().enumerate() {
        if idx > 0 {
            hm.insert(arg.clone(), idx);
        }
    }

    if hm.is_empty() {
        // if there's no filters, return directly
        process::exit(1);
    }

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        if let Ok(line) = ln {
            // don't print the line if filter condition is satisfied
            if !hm.contains_key(&line) {
                println!("{}", line);
            }
        }
    }

    // normal case
    process::exit(0);
}
