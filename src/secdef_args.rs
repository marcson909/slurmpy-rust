use std::{time::Duration, fmt::Error};
use chrono::{NaiveDate, ParseError};
use clap::{Args, Command, Subcommand, ValueEnum};
use serde::Deserialize;


#[derive(Args, Debug)]
pub struct SecDefCommand {
    #[command(flatten)]
    pub args: SecDefArgs,

    #[command(subcommand)]
    pub command: SecDefSubCommands,
}


/// Build a SecDef command
#[derive(Args, Debug)]
pub struct SecDefArgs {
    /// The path to the config file to read
    #[arg(short = 'c', long = "config", default_value = "./test_config.toml")]
    pub config_path: Option<std::path::PathBuf>,

    /// The trade date to run for
    #[arg(long = "trade-date", value_parser = parse_tradedate)]
    pub trade_date: Option<NaiveDate>,

}

impl SecDefArgs {
    pub fn to_vec(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(config) = &self.config_path {
            args.push(format!("--config {}", config.to_string_lossy()));
        }
        if let Some(tradedate) = &self.trade_date {
            args.push(format!("--trade-date {}", tradedate));
        }
        args
    }
}

fn parse_tradedate(arg: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(arg, "%Y%m%d")
}

#[derive(Debug, Subcommand)]
pub enum SecDefSubCommands {
    /// Execute a squeue command
    Normalize(NormalizeArgs),
}


/// Build a SecDef command
#[derive(Args, Debug)]
pub struct NormalizeArgs {
    /// Whether this should run optimized for slurm
    #[arg(short = 's', long = "slurm")]
    pub slurm: bool,
    pub ntype: NormalizeType,

}

#[derive(Clone, Debug, ValueEnum)]
pub enum NormalizeType {
    Source,
    Channel,
}