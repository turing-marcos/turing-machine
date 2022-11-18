// extern crate pest;
// #[macro_use]
// extern crate pest_derive;
use std::fs;

use turing_machine::TuringMachine;

fn main() {
    // let successful_parse = CSVParser::parse(Rule::final_state, "F={q2};");
    // println!("{:?}", successful_parse);

    let unparsed_file = fs::read_to_string("Examples/Example1.tm").expect("cannot read file");
    let tm = TuringMachine::new(&unparsed_file);
    println!("{}", tm);
}
