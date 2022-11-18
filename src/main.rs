use std::fs;
use std::io;
use std::path::PathBuf;
use clap::Parser as clap_parser;
use turing_machine::TuringMachine;

#[derive(clap_parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    /// Option: -f --file: Specify a file with the instructions.
    #[clap(long, short, help = "Specify a file with instructions.")]
    file: Option<PathBuf>,

    /// Option: -c --csv: Output in csv format.
    #[clap(long, short, default_value = "true", help = "Output in csv format.")]
    gui: bool,
}

fn main() {
    let args = Cli::parse();

    if let Some(file) = args.file {
        let unparsed_file = fs::read_to_string(file).expect("cannot read file");
        let tm = TuringMachine::new(&unparsed_file);
        run_machine(tm);
    } else {
        panic!("No program was provided");
    }
}


fn run_machine(mut tm: TuringMachine) {
    println!("{}", tm.to_string());
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    loop {
        tm.step();
        println!("{}", tm.to_string());

        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if tm.finished() {
            break;
        }
    }

    tm.step();
    println!("{}", tm.to_string());

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    println!("Result: {}", tm.tape_value());
}
