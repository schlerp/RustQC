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
    let qual_by_pos = lib::processing::process_fastq(&args.path);
    lib::report::print_report_header(&args.path);
    lib::report::print_qual_by_position_report(qual_by_pos);
}
