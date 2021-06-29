mod pretty_print;

use sec_data_parser::parse_submission;
use std::fs::File;
use std::io::BufReader;

use crate::pretty_print::PrettyPrint;
use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Describe(DescribeCommand),
}

#[derive(Clap)]
struct DescribeCommand {
    file: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Describe(DescribeCommand { file }) => {
            let mut reader = BufReader::new(File::open(file).unwrap());
            let submission = parse_submission(&mut reader).unwrap();

            submission.pretty_print();
        }
    }
}
