mod instruction;
mod turing;

pub use instruction::TuringInstruction;
pub use turing::{Rule, TuringMachine, TuringParser};

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::Rule;
    use crate::TuringMachine;
    use crate::TuringParser;
    use pest::Parser;

    #[test]
    fn parse_description() {
        let test = "/// a + b\r\n";
        let mut res = match TuringParser::parse(Rule::description, test) {
            Ok(v) => v,
            Err(e) => panic!("Parsing error: {}", e),
        };
        assert_eq!(res.next().unwrap().as_span().as_str(), test)
    }

    #[test]
    fn parse_tape() {
        let test = "{[q0]11111011};";
        let mut res = match TuringParser::parse(Rule::tape, test) {
            Ok(v) => v,
            Err(e) => panic!("Parsing error: {}", e),
        };
        assert_eq!(res.next().unwrap().as_span().as_str(), test)
    }

    #[test]
    fn parse_final_state() {
        let test = "F = {q2};";

        let mut res = match TuringParser::parse(Rule::final_state, test) {
            Ok(v) => v,
            Err(e) => panic!("Parsing error: {}", e),
        };
        assert_eq!(res.next().unwrap().as_span().as_str(), test)
    }

    #[test]
    fn parse_instruction() {
        let test = "(q0, 1, 0, R, q1);";

        let mut res = match TuringParser::parse(Rule::instruction, test) {
            Ok(v) => v,
            Err(e) => panic!("Parsing error: {}", e),
        };
        assert_eq!(res.next().unwrap().as_span().as_str(), test)
    }

    #[test]
    fn parse_file() {
        let unparsed_file = fs::read_to_string("Examples/Example1.tm").expect("cannot read file");
        let tm = TuringMachine::new(&unparsed_file);

        assert_eq!(
            tm.to_string(),
            "0 0 0 1 1 1 1 1 0 1 1 \n      ^               "
        )
    }
}
