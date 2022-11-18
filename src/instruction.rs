use crate::Rule;
use pest::iterators::Pairs;

#[derive(Debug, Clone)]
pub struct TuringInstruction {
    pub from_state: String,
    pub from_value: bool,
    pub to_value: bool,
    pub to_state: String,
}

impl TuringInstruction {
    pub fn from(mut code: Pairs<Rule>) -> Self {
        Self {
            from_state: String::from(code.next().unwrap().as_span().as_str()),
            from_value: code.next().unwrap().as_span().as_str() == "1",
            to_value: code.next().unwrap().as_span().as_str() == "1",
            to_state: String::from(code.next().unwrap().as_span().as_str()),
        }
    }
}
