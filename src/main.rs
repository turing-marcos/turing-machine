#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(target_arch = "wasm32"))]
use {
    clap::Parser as clap_parser,
    env_logger,
    log::trace,
    std::{fs, io, path::PathBuf},
    turing_machine::{turing::Rule, windows::ErrorWindow},
};

use turing_machine::{turing::TuringMachine, MyApp};

#[cfg(not(target_arch = "wasm32"))]
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

    /// Option: -i --interactive: print the machine result interactively (step by step) instead of printing directly the result.
    /// Note: this option is only available in the CLI mode.
    #[clap(
        long,
        short,
        default_value = "false",
        help = "print the machine result interactively (step by step).\nNote: this option is only available in the CLI mode."
    )]
    interactive: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
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
        Err(_e) => {
            //handle_error(e, file);
            std::process::exit(1);
        }
    };

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(MyApp::new(tm, cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    if let Some(file) = args.file {
        trace!("File provided: {:?}", file);

        if !args.cli {
            trace!("The machine will run in GUI mode");
            run_machine_gui(file);
        } else {
            trace!("The machine will run in CLI mode");
            run_machine_cli(file, args.interactive);
        }
    } else {
        trace!("No file provided, opening file picker in the current folder");

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

#[cfg(not(target_arch = "wasm32"))]
fn load_icon(path: &str) -> Option<eframe::IconData> {
    use log::error;

    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            error!("{}", e);
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
fn handle_error(e: pest::error::Error<Rule>, file: PathBuf) {
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
fn run_machine_cli(file: PathBuf, interactive: bool) {
    let unparsed_file = fs::read_to_string(&file).expect("cannot read file");
    let mut tm = match TuringMachine::new(&unparsed_file) {
        Ok(t) => t,
        Err(e) => {
            TuringMachine::handle_error(e);
            std::process::exit(1);
        }
    };

    if !interactive {
        let res = tm.final_result();
        println!("After {} steps, the result is: {}", res.0, res.1);
        std::process::exit(0);
    }

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
