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

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
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
        let path = std::env::current_dir().unwrap();

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

fn run_machine_gui(file: PathBuf) {
    let unparsed_file = fs::read_to_string(&file).expect("cannot read file");
    let tm = TuringMachine::new(&unparsed_file);

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
}

fn run_machine_cli(file: PathBuf) {
    let unparsed_file = fs::read_to_string(&file).expect("cannot read file");
    let mut tm = TuringMachine::new(&unparsed_file);

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
