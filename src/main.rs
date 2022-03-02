use clap::Parser;

mod lib;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    lib::process_fastq(args.path);
}