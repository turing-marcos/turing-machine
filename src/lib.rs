mod instruction;
mod turing;
mod turing_widget;
mod window;

pub use instruction::TuringInstruction;
pub use turing::{Rule, TuringMachine, TuringParser};
pub use turing_widget::TuringWidget;
pub use window::MyApp;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::Rule;
    use crate::TuringMachine;
    use crate::TuringParser;
    use pest::{consumes_to, parses_to};

    #[test]
    fn parse_description() {
        let test = "/// a + b\r\n";

        parses_to! {
            parser: TuringParser,
            input: test,
            rule: Rule::description,
            tokens: [
                description(0, 11),
            ]
        }
    }

    #[test]
    fn parse_tape() {
        let test = "{[q0]111011};";

        parses_to! {
            parser: TuringParser,
            input: test,
            rule: Rule::tape,
            tokens: [
                tape(0, 13, [
                    initial_state(1, 5, [
                        state(2, 4)
                    ]),
                    value(5, 6),
                    value(6, 7),
                    value(7, 8),
                    value(8, 9),
                    value(9, 10),
                    value(10, 11),
                ]),
            ]
        }
    }

    #[test]
    fn parse_final_state() {
        let test = "F = {q2};";

        parses_to! {
            parser: TuringParser,
            input: test,
            rule: Rule::final_state,
            tokens: [
                final_state(0, 9, [
                    state(5, 7)
                ])
            ]
        }
    }

    #[test]
    fn parse_instruction() {
        let test = "(q0, 1, 0, R, q1);";

        parses_to! {
            parser: TuringParser,
            input: test,
            rule: Rule::instruction,
            tokens: [
                instruction(0, 18, [
                    state(1, 3),
                    value(5, 6),
                    value(8, 9),
                    movement(11, 12),
                    state(14, 16)
                ]),
            ]
        }
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
