use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn wrapping_paper_req(line: String) -> (u64, u64) {
    let mut dim: Vec<u64> = Vec::new();
    let dim_str: Vec<&str> = line.split("x").collect();
    for d in &dim_str {
        dim.push(d.parse().unwrap());
    }
    dim.sort();

    (3*dim[0]*dim[1] + 2*dim[0]*dim[2] + 2*dim[1]*dim[2],
     2*dim[0]+2*dim[1] + dim[0]*dim[1]*dim[2])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).ok().expect("could not open file");
    let reader = BufReader::new(f);

    let mut paper: u64 = 0;
    let mut ribbon: u64 = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mat = wrapping_paper_req(line);
                paper += mat.0;
                ribbon += mat.1;
            },
            Err(e)   => panic!(e)
        }
    }
    println!("{} {}",paper,ribbon);
}
