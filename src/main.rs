#![allow(unused)]
use std::process::Stdio;
use std::{error::Error, println, writeln};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand, Command};
use secdef_args::SecDefArgs;
use slurmpy_rust::print_command_str;
use squeue_args::SqueueArgs;
mod config;
use crate::config::{DefaultConfig, MyConfig};
mod slurm;
mod squeue_args;
mod secdef_args;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help = true)]
struct Args {

    // /// The pattern to look for
    // pattern: String,
    // /// The path to the file to read
    // path: std::path::PathBuf,

    /// squeue arguments
    #[command(flatten)]
    sa: SqueueArgs,

    #[command(subcommand)]
    command: Commands,

    // /// The text to show if no pattern was matched
    // #[arg(required(true))]
    // text: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Execute a squeue command
    Squeue(SqueueArgs),
    Secdef(SecDefArgs),
}


fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run (args: Args)  -> slurmpy_rust::DynResult<()>{
    // let maybe_pattern = args.pattern;
    // let maybe_path = args.path.to_string_lossy();
    // let cfg: MyConfig = confy::load_path(&args.config_path)?;
    // println!("pattern: {}", maybe_pattern);
    // println!("path: {}", maybe_path);
    // println!("{:#?}", cfg);

    // match slurmpy_rust::open(&maybe_path) {
    //     Err(e) => eprintln!("{}: {}", maybe_path, e),
    //     Ok(content) => {
    //         slurmpy_rust::find_matches(
    //             content,
    //             &maybe_pattern,
    //             &args.text.join(" "),
    //             &mut std::io::stdout()
    //         );
    //     }
    // }

    match args.command {
        Commands::Secdef(secdef_args) => {
            // let secdef_str = format!("{:?}",secdef_args.to_vec());
            let secdef_str = &secdef_args.to_vec().join(" ");
            // let secdef_command = Command::new("secdef")
            //     .args(secdef_args.to_vec().into_iter());
            // secdef_command
                // .stdout(Stdio::null())
                // .stderr(Stdio::null())
                // .spawn()
                // .expect("failed to execute secdef");
            println!("{:?}", secdef_args);
            let command_str = format!("secdef {}", &secdef_str);
            print_command_str(&mut std::io::stdout(), &command_str);
        },
        Commands::Squeue(squeue_args) => {
            let squeue_str = format!("{:?}",squeue_args.to_vec());
            // println!("{}", squeue_str)
            print_command_str(&mut std::io::stdout(), &squeue_str);
        }
    }
    Ok(())
}

// ---------------------------------------------------
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    slurmpy_rust::find_matches("lorem ipsum\ndolor sit amet".as_bytes(), "lorem", "test_fallback_str",&mut result);
    assert_eq!(result, b"0 lorem ipsum\n");
}

// ---------------------------------------------------
#[test]
fn return_fallback_str() {
    let mut result = Vec::new();
    slurmpy_rust::find_matches("lorem ipsum\ndolor sit amet".as_bytes(), "missing", "test_fallback_str",&mut result);
    assert_eq!(result, b"test_fallback_str\n");
}