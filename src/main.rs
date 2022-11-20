use clap::Parser as clap_parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use turing_machine::MyApp;
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

    /// Option: -c --cli: Output in the command-line instead of the GUI.
    #[clap(
        long,
        short,
        default_value = "false",
        help = "Output in the command-line."
    )]
    cli: bool,
}

fn main() {
    let args = Cli::parse();

    if let Some(file) = args.file {
        let unparsed_file = fs::read_to_string(&file).expect("cannot read file");
        let tm = TuringMachine::new(&unparsed_file);

        if !args.cli {
            let options = eframe::NativeOptions {
                drag_and_drop_support: true,
                ..Default::default()
            };
            eframe::run_native(
                &format!(
                    "Turing Machine: {:?}",
                    file.file_name()
                        .unwrap_or(std::ffi::OsStr::new("User input"))
                ),
                options,
                Box::new(|cc| Box::new(MyApp::new(tm, cc))),
            );
        } else {
            run_machine(tm);
        }
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
