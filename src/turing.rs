use pest::Parser;
use pest_derive::Parser;
use std::{collections::HashMap, fmt::Write};

use crate::{instruction::Movement, TuringInstruction};

#[derive(Parser)]
#[grammar = "turing.pest"]
pub struct TuringParser;

#[derive(Debug, Clone)]
pub struct TuringMachine {
    pub instructions: HashMap<(String, bool), TuringInstruction>,
    pub final_states: Vec<String>,
    pub current_state: String,
    pub tape_position: usize,
    pub tape: Vec<bool>,
}

impl TuringMachine {
    pub fn new(code: &str) -> Self {
        let mut instructions: HashMap<(String, bool), TuringInstruction> = HashMap::new();
        let mut final_states: Vec<String> = Vec::new();
        let mut current_state: String = String::new();
        let mut tape: Vec<bool> = Vec::new();
        let mut tape_position = 0;

        let file = TuringParser::parse(Rule::file, code)
            .expect("unsuccessful parse") // unwrap the parse result
            .next()
            .unwrap(); // get and unwrap the `file` rule; never fails

        for record in file.into_inner() {
            match record.as_rule() {
                Rule::description => println!("{}", record.into_inner().as_str()),
                Rule::tape => {
                    let mut tmp = record.into_inner();
                    // FIXME: The state could not be the first item
                    current_state = String::from(
                        tmp.next()
                            .unwrap()
                            .as_str()
                            .replace("[", "")
                            .replace("]", ""),
                    );
                    tape = tmp
                        .map(|v| v.as_span().as_str() == "1")
                        .collect::<Vec<bool>>();

                    // println!("Initial state: {}", current_state);
                    // println!("Tape: {:?}", tape);
                }
                Rule::final_state => {
                    final_states = record
                        .into_inner()
                        .map(|v| String::from(v.as_span().as_str()))
                        .collect();
                    // println!("The final tape state is {:?}", final_states);
                }
                Rule::instruction => {
                    let tmp = TuringInstruction::from(record.into_inner());
                    instructions.insert(
                        (tmp.from_state.clone(), tmp.from_value.clone()),
                        tmp.clone(),
                    );

                    // println!("Found instruction {:?}", tmp);
                }
                Rule::comment => {
                    // println!("Found comment: {}", record.into_inner().as_str());
                }
                Rule::empty => {
                    // println!("Empty stuff");
                }
                Rule::EOI => {
                    // println!("End of file");
                }
                _ => {
                    println!("Unhandled: {}", record.into_inner().as_str());
                }
            }
        }

        while tape_position <= 2 {
            tape.insert(0, false);
            tape_position += 1;
        }

        while tape_position >= tape.len() - 3 {
            tape.push(false);
        }

        Self {
            instructions,
            final_states,
            current_state,
            tape_position,
            tape,
        }
    }

    pub fn step(&mut self) {
        let current_val: bool = self.tape[self.tape_position];
        let index = (self.current_state.clone(), current_val);

        let Some(instruction) = self.instructions.get(&index) else {
            panic!("No instruction given for state ({}, {})", self.current_state.clone(), current_val);
        };

        self.tape[self.tape_position] = instruction.to_value;

        match instruction.movement {
            Movement::LEFT => {
                if self.tape_position == 0 {
                    self.tape.insert(0, false);
                } else {
                    self.tape_position -= 1;
                }
            }
            Movement::RIGHT => {
                if self.tape_position == self.tape.len() - 1 {
                    self.tape.push(false);
                }

                self.tape_position += 1;
            }
            Movement::HALT => {}
        }

        while self.tape_position <= 2 {
            self.tape.insert(0, false);
            self.tape_position += 1;
        }

        while self.tape_position >= self.tape.len() - 3 {
            self.tape.push(false);
        }

        self.current_state = instruction.to_state.clone();
    }

    pub fn finished(&self) -> bool {
        return self.final_states.contains(&self.current_state);
    }

    pub fn to_string(&self) -> String {
        let mut tmp1 = String::new();
        let mut tmp2 = String::new();

        for (i, v) in self.tape.iter().enumerate() {
            write!(&mut tmp1, "{} ", if v.clone() { "1" } else { "0" }).unwrap();

            if i == self.tape_position {
                tmp2 += "^ ";
            } else {
                tmp2 += "  ";
            }
        }

        format!("{}\n{}", tmp1, tmp2)
    }

    pub fn tape_value(&self) -> u32 {
        self.tape.iter().map(|v| if *v {1} else {0}).sum()
    }
}