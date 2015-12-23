extern crate md5;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut i_5: u64 = 0;
    let mut i_6: u64 = 0;
    let mut found_5 = false;
    let mut found_6 = false;

    for i in 0..std::u64::MAX {
        let key_i = args[1].clone() + &i.to_string();
        let key_i_bytes = key_i.into_bytes();
        let hash = md5::compute(&*key_i_bytes.into_boxed_slice());

        if 0 == hash[0] && 0 == hash[1] && 0 == (hash[2] & 0xf0) { 
            if 0 == hash[2] && !found_6 {
                i_6 = i;
                found_6 = true;
            }

            if !found_5 {
                i_5 = i;
                found_5 = true;
            }
        }

        if found_5 && found_6 {
            println!("starts with 5 zeros {}",i_5);
            println!("starts with 6 zeros {}",i_6);
            break;
        }
    }
}
