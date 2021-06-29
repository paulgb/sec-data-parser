mod pretty_print;

use sec_data_parser::{parse_submission, Submission};
use std::fs::{File, read_dir};
use std::io::BufReader;

use crate::pretty_print::PrettyPrint;
use clap::{AppSettings, Clap};
use std::path::{PathBuf, Path};

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

fn read_submission(path: &Path) -> Submission {
    let mut reader = BufReader::new(File::open(path).unwrap());
    parse_submission(&mut reader).unwrap()
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Describe(DescribeCommand { file }) => {
            let submission = read_submission(&file);

            submission.pretty_print();
        }
        SubCommand::Check(CheckCommand {dir}) => {
            for file in read_dir(dir).unwrap() {
                let path = file.unwrap().path();
                println!("{:?}", &path);

                read_submission(&path);
            }
        }
    }
}
