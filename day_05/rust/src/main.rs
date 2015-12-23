use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn is_nice_1(candidate: String) -> bool {
    if candidate.contains("ab") {
        return false;
    }
    if candidate.contains("cd") {
        return false;
    }
    if candidate.contains("pq") {
        return false;
    }
    if candidate.contains("xy") {
        return false;
    }
    let mut repeated: bool = false;
    let mut vowel_count: u64 = 0;

    let mut ci = candidate.chars().peekable();
    while !ci.peek().is_none() {
        let c = ci.next().unwrap();
        if 3 > vowel_count {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => { vowel_count += 1; },
                _ => {}
            }
        }

        if !repeated && !ci.peek().is_none() {
            if c == *ci.peek().unwrap() {
                repeated = true;
            }
        }

        if 3 <= vowel_count && repeated {
            break;
        }
    }

    if 3 > vowel_count || !repeated {
        return false;
    }

    true
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct CharPair {
    c0: char,
    c1: char,
}


fn is_nice_2(candidate: String) -> bool {
    let mut repeated = false;
    let mut double_digraph = false;

    let mut ci = candidate.chars().peekable();
    let mut prev_char = ci.next().unwrap();
    
    let mut digraphs: HashMap<CharPair,u8> = HashMap::new();
    let mut start_pos: u8 = 0;

    while !ci.peek().is_none() {
        let curr_char = ci.next().unwrap();

        if !repeated {
            match ci.peek() {
                Some(next_char) => {
                    if prev_char == *next_char {
                        repeated = true;
                    }
                },
                _ => {}
            }
        }

        if !double_digraph {
            let cp = CharPair{c0:prev_char,c1:curr_char};
            if digraphs.contains_key(&cp) {
                if let Some(prev_start_pos) = digraphs.get_mut(&cp) {
                    if *prev_start_pos < start_pos-1 {
                        double_digraph = true;
                    }
                }
            }
            else {
                digraphs.insert(cp,start_pos);
            }
        }

        if repeated && double_digraph {
            return true; 
        }

        start_pos += 1;
        prev_char = curr_char;
    }

    println!("{} {} {}",candidate,repeated,double_digraph);
    repeated && double_digraph
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).ok().expect("could not open file");
    let reader = BufReader::new(f);

    let mut nice_count_1: u64 = 0;
    let mut nice_count_2: u64 = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if is_nice_1(line.clone()) {
                    nice_count_1 += 1;
                }
                if is_nice_2(line) {
                    nice_count_2 += 1;
                }
            },
            Err(err) => panic!(err)
        }
    }

    println!("{}",nice_count_1);
    println!("{}",nice_count_2);
}
