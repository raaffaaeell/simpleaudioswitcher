use clap::Parser;
mod output;

/// Simple program to switch audio stuff.
/// Supports wpctl (wireplumber) or pactl (pipewire/pulseaudio)
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Which cli to use, pac (pactl) or wp (wpctl)
    #[clap(short, long)]
    cli: String,
}

fn main() {
    let args = Args::parse();
    match args.cli.as_str() {
        "pac" => output::pactloutputs::set_default_sink_pactl(),
        "wp" => output::wpctl::get_outputs_wpctl(),
        _ => eprintln!("Invalid cli argument")
    }
}
