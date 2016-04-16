use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
enum Signal {
    Name(String),
    Value(u16),
}

#[derive(Debug)]
enum Operation {
    PASS,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

#[derive(Debug)]
struct Node<'a> {
    in0: Option<Rc<RefCell<&'a Signal>>>,
    in1: Option<Rc<RefCell<&'a Signal>>>,
    out: Rc<RefCell<&'a Signal>>,
    op: Operation,
}


//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
fn parse_signal_from_input<'a>(map: &'a mut BTreeMap<String,Signal>, name: &str) -> Rc<RefCell<&'a Signal>> {
    map.insert(name.to_string(),match name.parse() {
        Ok(val) => Signal::Value(val),
        _ => Signal::Name(name.to_string())
    });

    Rc::new(RefCell::new(map.get(name).expect("error with map")))
}




//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        panic!("Arguments Required: <path to file>");
    }
    let f = File::open(&args[1]).ok().expect("could not open file");
    let reader = BufReader::new(f);

    // build list of nodes and signals that need to be resolved
    let mut signals: BTreeMap<String,Signal> = BTreeMap::new();
    let mut nodes: Vec<Node> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // split input and output string
                let v: Vec<&str> = line.split(" -> ").collect();

                // handle output signal string
                let out_sig = parse_signal_from_input(&mut signals,&v[1]);

                // split input string
                let vi: Vec<&str> = v[0].split_whitespace().collect();
                let node = match vi.len() {
                    1 => { // A passthrough signal
                        Node{in0: Some(parse_signal_from_input(&mut signals,&vi[0])),
                             in1: None, //None::<Rc<RefCell<Signal>>,
                             out: out_sig.clone(),
                              op: Operation::PASS}
                    },
                    2 => { // A NOT gate
                        Node{in0: Some(parse_signal_from_input(&mut signals,&vi[1])),
                             in1: None,
                             out: out_sig.clone(),
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

                        Node{in0: Some(parse_signal_from_input(&mut signals,&vi[0])),
                             in1: Some(parse_signal_from_input(&mut signals,&vi[2])),
                             out: out_sig.clone(),
                              op: op}
                    }
                };


                //println!("{:?} : {:?}",out_sig,node);
                nodes.push(node);
            }
            Err(err) => panic!(err)
        }
    }

    //println!("{}",get_signal_value(&out_sig,&sig));
    let mut len = nodes.len();
    while 0 < len {
        for ix in 0..len {
            nodes.swap_remove(ix);
            len = nodes.len();
            break;
        }
    }
}
