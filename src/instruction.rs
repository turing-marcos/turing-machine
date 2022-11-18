use std::str::FromStr;

use crate::Rule;
use pest::iterators::Pairs;

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    RIGHT,
    LEFT,
    HALT,
}

impl std::str::FromStr for Movement {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "R" => Ok(Self::RIGHT),
            "L" => Ok(Self::LEFT),
            _ => Ok(Self::HALT),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TuringInstruction {
    pub from_state: String,
    pub from_value: bool,
    pub to_value: bool,
    pub movement: Movement,
    pub to_state: String,
}

impl TuringInstruction {
    pub fn from(mut code: Pairs<Rule>) -> Self {
        Self {
            from_state: String::from(code.next().unwrap().as_span().as_str()),
            from_value: code.next().unwrap().as_span().as_str() == "1",
            to_value: code.next().unwrap().as_span().as_str() == "1",
            movement: Movement::from_str(code.next().unwrap().as_span().as_str()).unwrap(),
            to_state: String::from(code.next().unwrap().as_span().as_str()),
        }
    }
}
