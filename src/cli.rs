use clap::{Parser, Subcommand, Command, Args};
use crate::{SecDefArgs, SecDefCommand, SecDefSubCommands};
use crate::SqueueArgs;
use crate::GreprArgs;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// cli arguments
    #[command(flatten)]
    pub args: CliArgs,

    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Args, Debug)]
pub struct CliArgs {
    #[arg(long)]
    pub test_arg: bool,
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Execute a squeue command
    Squeue(SqueueArgs),
    Secdef(SecDefCommand),
    Grepr(GreprArgs),
}