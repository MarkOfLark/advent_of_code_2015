use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Signal {
    None,
    Value(u16),
    Name(String),
}

#[derive(Debug)]
enum Operation {
    None,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

#[derive(Debug)]
struct Node {
    in0: Signal,
    in1: Signal,
    op: Operation,
}


//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
fn parse_signal_from_input(name: &str) -> Signal {
    match name.parse() {
        Ok(val) => Signal::Value(val),
        Err(err)=> Signal::Name(name.to_string())
    }
}


//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
fn get_signal_value(map: &HashMap<String,Node>, sig: &Signal) -> u16 {
    match *sig {
        Signal::Value(val)     => val,
        Signal::None           => panic!("cannot get value for None signal"),
        Signal::Name(ref name) => {
            print!("{}: ", name);
            let node = map.get(name).expect("could not find sig name in map");
            println!("{:?}",node);

            match node.op {
                Operation::None => {
                    get_signal_value(map, &node.in0)
                }
                Operation::NOT => {
                    !get_signal_value(map, &node.in0)
                }
                _ => {
                    let val0 = get_signal_value(map, &node.in0);
                    let val1 = get_signal_value(map, &node.in1);
                    match node.op {
                        Operation::LSHIFT => val0 << val1,
                        Operation::RSHIFT => val0 >> val1,
                        Operation::AND    => val0 & val1,
                        Operation::OR     => val0 | val1,
                        _                 => panic!("unreachable")
                    } // end match on 2 operand operations
                }
            } // end match on all operations
        } // end handling of named signal
    } // end match on signal type
} // end get_signal_value


//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
fn main() {
    let args: Vec<String> = env::args().collect();
    if 3 > args.len() {
        panic!("Arguments Required: <path to file> <signal name>");
    }
    let f = File::open(&args[1]).ok().expect("could not open file");
    let reader = BufReader::new(f);
    let sig = Signal::Name(args[2].to_string());

    // Read in all the outputs sorted alphabetically and then asign
    // an index to each output signal starting at 0
    let mut out_sig: HashMap<String,Node> = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // split input and output string
                let v: Vec<&str> = line.split(" -> ").collect();

                // split input string
                let vi: Vec<&str> = v[0].split_whitespace().collect();
                let node = match vi.len() {
                    1 => { // A passthrough signal
                        Node{in0: parse_signal_from_input(&vi[0]),
                             in1: Signal::None,
                              op: Operation::None}
                    },
                    2 => { // A NOT gate
                        Node{in0: parse_signal_from_input(&vi[1]),
                             in1: Signal::None,
                              op: Operation::NOT}
                    },
                    _ => { // Any of the other nodes types
                        let op = match vi[1] {
                            "AND"    => Operation::AND,
                            "OR"     => Operation::OR,
                            "LSHIFT" => Operation::LSHIFT,
                            "RSHIFT" => Operation::RSHIFT,
                            _        => panic!("unrecognized operation")
                        };

                        Node{in0: parse_signal_from_input(&vi[0]),
                             in1: parse_signal_from_input(&vi[2]),
                              op: op}
                    }
                };
                out_sig.insert(v[1].to_string(),node);
            }
            Err(err) => panic!(err)
        }
    }

    println!("{}",get_signal_value(&out_sig,&sig));
}
