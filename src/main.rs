#![allow(unused)]
use std::process::Stdio;
use std::{error::Error, println, writeln};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand, Command};
use cli::{Cli, CliArgs, Commands};
use secdef_args::{SecDefArgs, SecDefCommand, SecDefSubCommands};
use slurmpy_rust::print_command_str;
use squeue_args::SqueueArgs;
use grepr_args::GreprArgs;
mod config;
use crate::config::{DefaultConfig, MyConfig};
mod slurm;
mod cli;
mod squeue_args;
mod secdef_args;
mod grepr_args;


fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run (cli: Cli)  -> slurmpy_rust::DynResult<()>{
    // let cfg: MyConfig = confy::load_path(&args.config_path)?;
    // println!("{:#?}", cfg);
    match cli.command {
        Commands::Secdef(secdef_command) => {
            let command_args = &secdef_command.args;
            let mut commands = command_args.to_vec();
            let subcommand = &secdef_command.command;
            match subcommand {
                SecDefSubCommands::Normalize(normalize_args) => {
                    commands.push("normalize".into());
                    if true == normalize_args.slurm{
                        commands.push("--slurm".into());
                    };
                    commands.push(format!("--ntype {:?}", normalize_args.ntype));
                }
            }
            println!("{:#?}", &secdef_command);
            let secdef_str = &commands.join(" ");
            let command_str = format!("secdef {}", &secdef_str);
            print_command_str(&mut std::io::stdout(), &command_str);
        },
        Commands::Squeue(squeue_args) => {
            let squeue_str = format!("{:?}",squeue_args.to_vec());
            // println!("{}", squeue_str)
            print_command_str(&mut std::io::stdout(), &squeue_str);
        }
        Commands::Grepr(grepr_args) => {
            let maybe_path = grepr_args.path.to_string_lossy();
            match slurmpy_rust::open(&maybe_path) {
                Err(e) => eprintln!("{}: {}", maybe_path, e),
                Ok(content) => {
                    slurmpy_rust::find_matches(
                        content,
                        &grepr_args.pattern,
                        &grepr_args.fallback_text.join(" "),
                        &mut std::io::stdout()
                    );
                }
            }
        },
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