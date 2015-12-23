use std::env;
use std::cmp;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

struct LightDisplay {
    state: [[i32;1000];1000]
}
impl LightDisplay {
    fn modify_range<M>(&mut self, r: &Vec<usize>, modify: M) 
    where M : Fn(i32) -> i32 {
        for row in r[2]..r[3]+1 {
            for col in r[0]..r[1]+1 {
                self.state[row][col] = modify(self.state[row][col]);
            }
        }
    }

    fn how_bright(&self) -> u64 {
        let mut brightness: u64 = 0;
        for row in 0..1000 {
            for col in 0..1000 {
                brightness += self.state[row][col] as u64;
            }
        }
        brightness
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).ok().expect("could not open file");
    let reader = BufReader::new(f);

    let mut my_house = LightDisplay{state: [[0i32; 1000]; 1000]};
    let mut my_dream_house = LightDisplay{state: [[0i32; 1000]; 1000]};

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let v: Vec<&str> = line.split(|c| c == ' ' || c == ',').collect();
                if line.starts_with("turn on") {
                    let r: Vec<usize> = vec![v[2].parse().unwrap(),
                                             v[5].parse().unwrap(),
                                             v[3].parse().unwrap(),
                                             v[6].parse().unwrap()];
                    my_house.modify_range(&r, |x| 1);
                    my_dream_house.modify_range(&r, |x| x+1);
                } 
                else if line.starts_with("turn off") {;
                    let r: Vec<usize> = vec![v[2].parse().unwrap(),
                                             v[5].parse().unwrap(),
                                             v[3].parse().unwrap(),
                                             v[6].parse().unwrap()];
                    my_house.modify_range(&r, |x| 0);
                    my_dream_house.modify_range(&r, |x| cmp::max(0,x-1));
                }
                else if line.starts_with("toggle") {;
                    let r: Vec<usize> = vec![v[1].parse().unwrap(),
                                             v[4].parse().unwrap(),
                                             v[2].parse().unwrap(),
                                             v[5].parse().unwrap()];
                    my_house.modify_range(&r, |x| 1-x);
                    my_dream_house.modify_range(&r, |x| x+2);
                }
                //println!("{:?}",v);
            },
            Err(err) => panic!(err)
        }
    }

    println!("{}",my_house.how_bright());
    println!("{}",my_dream_house.how_bright());

}
