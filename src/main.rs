use clap::Parser;

mod lib;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let (qual_by_pos, base_by_pos) = lib::processing::process_fastq(&args.path);
    lib::report::print_report_header(&args.path);
    lib::report::print_qual_by_position_report(&qual_by_pos);
    lib::report::print_base_overall_report(&base_by_pos);
    lib::report::print_base_by_position_report(&base_by_pos);
}
