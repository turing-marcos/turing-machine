use pest::Parser;
use pest_derive::Parser;
use std::{collections::HashMap, fmt::Display};

use crate::TuringInstruction;

#[derive(Parser)]
#[grammar = "turing.pest"]
pub struct TuringParser;

#[derive(Debug, Clone)]
pub struct TuringMachine {
    pub instructions: HashMap<(String, bool), TuringInstruction>,
    pub final_states: Vec<String>,
    pub current_state: String,
    pub tape_position: i32,
    pub tape: Vec<bool>,
}

impl Display for TuringMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = String::new();

        for (i, v) in self.tape.iter().enumerate() {
            write!(f, "{} ", if v.clone() { "1" } else { "0" })?;

            if i == self.tape_position.try_into().unwrap() {
                tmp += "^ ";
            } else {
                tmp += "  ";
            }
        }

        write!(f, "\n{}", tmp)
    }
}

impl TuringMachine {
    pub fn new(code: &str) -> Self {
        let mut instructions: HashMap<(String, bool), TuringInstruction> = HashMap::new();
        let mut final_states: Vec<String> = Vec::new();
        let mut current_state: String = String::new();
        let mut tape: Vec<bool> = Vec::new();

        let file = TuringParser::parse(Rule::file, code)
            .expect("unsuccessful parse") // unwrap the parse result
            .next()
            .unwrap(); // get and unwrap the `file` rule; never fails

        for record in file.into_inner() {
            match record.as_rule() {
                Rule::tape => {
                    let mut tmp = record.into_inner();
                    // FIXME: The state could not be the first item
                    current_state = String::from(tmp.next().unwrap().as_str());
                    tape = tmp
                        .map(|v| v.as_span().as_str() == "1")
                        .collect::<Vec<bool>>();

                    // println!("Initial state: {}", current_state);
                    // println!("Tape: {:?}", tape);
                },
                Rule::final_state => {
                    final_states = record
                        .into_inner()
                        .map(|v| String::from(v.as_span().as_str()))
                        .collect();
                    // println!("The final tape state is {:?}", final_states);
                },
                Rule::instruction => {
                    let tmp = TuringInstruction::from(record.into_inner());
                    instructions.insert(
                        (tmp.from_state.clone(), tmp.from_value.clone()),
                        tmp.clone(),
                    );

                    // println!("Found instruction {:?}", tmp);
                },
                Rule::comment => {
                    // println!("Found comment: {}", record.into_inner().as_str());
                },
                Rule::empty => {
                    // println!("Empty stuff");
                },
                Rule::EOI => {
                    // println!("End of file");
                },
                _ => {
                    println!("Unhandled: {}", record.into_inner().as_str());
                }
            }
        }

        Self {
            instructions,
            final_states,
            current_state,
            tape_position: 0,
            tape,
        }
    }
}
