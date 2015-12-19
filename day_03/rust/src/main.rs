use std::env;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
struct House {
    north: i64,
    east: i64,
}
impl Copy for House {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_santas = args[1].parse().unwrap();
    let directions: Vec<char> = args[2].chars().collect();

    let mut latest_house: Vec<House> = Vec::new();
    for s in 0..num_santas {
        latest_house.push(House{north:0,east:0});
    }

    let mut the_hood = HashMap::new();
    let mut turn = 0;
    the_hood.insert(latest_house[0],2);
    for d in directions {
        let ref mut lh = latest_house[turn%num_santas];
        turn += 1;

        match d {
            '^' => lh.north += 1,
            '>' => lh.east += 1,
            'v' => lh.north -= 1,
            '<' => lh.east -= 1,
             _  => panic!("invalid input")
        }

        if the_hood.contains_key(lh) {
            if let Some(deliveries) = the_hood.get_mut(lh) {
                *deliveries += 1;
            }
        }
        else {
            the_hood.insert(House{north:lh.north,east:lh.east},1);
        }
    }

    println!("{} unique houses",the_hood.len());
}
