use clap::Parser;
use shotty::commands::args::Args;
use shotty::commands::clipboard::copy_to_clipboard;
use shotty::commands::config::load_config;
use shotty::commands::screenshot::take_screenshot;

fn main() {
    let args = Args::parse();
    let config = load_config();

    let filename = take_screenshot(args.fullscreen, args.output, &config);
    if !filename.is_empty() {
        copy_to_clipboard(&filename);
    }
}
