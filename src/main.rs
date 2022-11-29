#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser as clap_parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use turing_machine::ErrorWindow;
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

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let unparsed_file = "/// a + b

{11111011};

I = {q0};
F = {q2};

(q0, 1, 0, R, q1);

(q1, 1, 1, R, q1);
(q1, 0, 0, R, q2);

(q2, 1, 0, H, q2);
(q2, 0, 0, H, q2);
";

    let tm = match TuringMachine::new(&unparsed_file) {
        Ok(t) => t,
        Err(e) => {
            //handle_error(e, file);
            std::process::exit(1);
        }
    };

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(MyApp::new(tm, cc))),
    )
    .expect("failed to start eframe");
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args = Cli::parse();

    if let Some(file) = args.file {
        if !args.cli {
            run_machine_gui(file);
        } else {
            run_machine_cli(file);
        }
    } else {
        let path = std::env::current_dir().unwrap_or_default();

        let res = rfd::FileDialog::new()
            .add_filter("TuringMachine", &["tm"])
            .set_directory(&path)
            .pick_files();

        match res {
            Some(file) => run_machine_gui(file[0].clone()),
            None => panic!("No file was chosen"),
        };
    }
}

fn load_icon(path: &str) -> Option<eframe::IconData> {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };

    Some(eframe::IconData {
        rgba: data,
        width: 32,
        height: 32,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn run_machine_gui(file: PathBuf) {
    let unparsed_file = fs::read_to_string(&file).expect("cannot read file");
    let tm = match TuringMachine::new(&unparsed_file) {
        Ok(t) => t,
        Err(e) => {
            handle_error(e, file);
            std::process::exit(1);
        }
    };

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        icon_data: load_icon("assets/icon.png"),
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
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_error(e: pest::error::Error<turing_machine::Rule>, file: PathBuf) {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        icon_data: load_icon("assets/icon.png"),
        ..Default::default()
    };

    eframe::run_native(
        &format!(
            "Turing Machine: {:?}",
            file.file_name()
                .unwrap_or(std::ffi::OsStr::new("User input"))
        ),
        options,
        Box::new(|cc| Box::new(ErrorWindow::new(e, file, cc))),
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn run_machine_cli(file: PathBuf) {
    let unparsed_file = fs::read_to_string(&file).expect("cannot read file");
    let mut tm = match TuringMachine::new(&unparsed_file) {
        Ok(t) => t,
        Err(e) => {
            TuringMachine::handle_error(e);
            std::process::exit(1);
        }
    };

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
