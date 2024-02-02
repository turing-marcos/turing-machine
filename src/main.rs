#![warn(clippy::all, rust_2018_idioms)]
#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(target_arch = "wasm32"))]
use {
    clap::Parser as clap_parser,
    log::{debug, error},
    std::{fs, io, path::PathBuf},
    turing_lib::TuringMachine,
    turing_machine::windows::ErrorWindow,
};

use eframe::egui;
use turing_machine::MyApp;

#[cfg(not(target_arch = "wasm32"))]
#[derive(clap_parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    /// Option: Specify a file with the instructions.
    #[clap(help = "Specify a file with instructions.")]
    file: Option<PathBuf>,

    /// Option: -c --cli: Output in the command-line instead of the GUI.
    #[clap(
        long,
        short,
        default_value_t = false,
        help = "Output in the command-line."
    )]
    cli: bool,

    /// Option: -i --interactive: print the machine result interactively (step by step) instead of printing directly the result.
    /// Note: this option is only available in the CLI mode.
    #[clap(
        long,
        short,
        default_value_t = false,
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

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "TuringMachineCanvas", // hardcode it
                web_options,
                Box::new(|cc| Box::new(MyApp::new(&None, cc).unwrap())),
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

    if args.cli {
        if let Some(file) = args.file {
            debug!("The machine will run in GUI mode");
            run_machine_cli(file, args.interactive);
        } else {
            error!("No file provided, exiting...");
            std::process::exit(1);
        }
    } else {
        run_machine_gui(args.file);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_machine_gui(file: Option<PathBuf>) {
    use eframe::egui::ViewportBuilder;
    use turing_machine::get_lang;

    let viewport: ViewportBuilder = egui::ViewportBuilder::default()
        .with_inner_size([900.0, 700.0])
        .with_drag_and_drop(true)
        .with_icon(
            eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")).unwrap(),
        );

    let options = eframe::NativeOptions {
        follow_system_theme: true,
        viewport,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    let file_name = match &file {
        Some(file_some) => file_some
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("User input")),
        None => std::ffi::OsStr::new("Example 1"),
    };

    match eframe::run_native(
        &format!("Turing Machine: {:?}", file_name),
        options,
        Box::new(move |cc| match MyApp::new(&file, cc) {
            Ok(w) => Box::new(w),
            Err(e) => Box::new(ErrorWindow::new(e, file, get_lang(), cc)),
        }),
    ) {
        Ok(_) => (),
        Err(e) => {
            error!("Error running eframe: {}", e);
            std::process::exit(1);
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
fn run_machine_cli(file: PathBuf, interactive: bool) {
    use turing_lib::TuringOutput;

    let u_code = fs::read_to_string(&file).expect("cannot read file");
    let mut tm = match TuringMachine::new(&u_code) {
        Ok((t, warnings)) => {
            for w in warnings {
                println!("\tWarning: {:?}", w);
            }

            t
        }
        Err(e) => {
            TuringMachine::handle_error(e);
            std::process::exit(1);
        }
    };

    if !interactive {
        let res = tm.final_result();
        match res {
            TuringOutput::Undefined(steps) => {
                println!("After {} steps, the result is: Undefined", steps);
            }
            TuringOutput::Defined((steps, value)) => {
                println!("After {} steps, the result is: {}", steps, value);
            }
        }
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
