mod pretty_print;

use sec_data_parser::parse_submission;
use std::fs::read_dir;

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
    Check(CheckCommand),
}

#[derive(Clap)]
struct DescribeCommand {
    file: PathBuf,
}

#[derive(Clap)]
struct CheckCommand {
    dir: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Describe(DescribeCommand { file }) => {
            let submission = parse_submission(&file).unwrap();

            submission.pretty_print();
        }
        SubCommand::Check(CheckCommand { dir }) => {
            for file in read_dir(dir).unwrap() {
                let path = file.unwrap().path();
                println!("{:?}", &path);

                parse_submission(&path).unwrap();
            }
        }
    }
}
