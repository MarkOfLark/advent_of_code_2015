use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let directions: Vec<char> = args[1].chars().collect();

    let mut count: i64 = 0;
    let mut floor: i64 = 0;
    let mut basement: i64 = 0;
    for f in directions {
        count += 1;
        match f {
            '(' => floor += 1,
            ')' => floor -= 1,
             _  => panic!("invalid input")
        }
        if 0 > floor && 0 == basement {
            basement = count;
        }
    }
    println!("{} {}",floor, basement);
}
