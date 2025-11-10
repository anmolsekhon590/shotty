use clap::Parser;
use shotty::commands::args::Args;
use shotty::commands::clipboard::copy_to_clipboard;
use shotty::commands::config::load_config;
use shotty::commands::deps::check_dependencies;
use shotty::commands::screenshot::take_screenshot;

fn main() {
    let args = Args::parse();

    // Check for required dependencies before proceeding
    if let Err(error_msg) = check_dependencies() {
        eprintln!("{}", error_msg);
        std::process::exit(1);
    }

    let config = load_config();

    let filename = take_screenshot(args.fullscreen, args.output, &config, args.freeze);
    if !filename.is_empty() {
        copy_to_clipboard(&filename);
    }
}
