// extern crate pest;
// #[macro_use]
// extern crate pest_derive;
use std::fs;
use std::io;
use turing_machine::TuringMachine;

fn main() {
    let unparsed_file = fs::read_to_string("Examples/Example1.tm").expect("cannot read file");
    let mut tm = TuringMachine::new(&unparsed_file);

    loop {
        println!("{}", tm.to_string());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        tm.step();
    }
}
