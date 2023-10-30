#![allow(unused)]
use std::{error::Error, println, writeln};
use std::format;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
mod config;

pub type DynResult<T> = Result<T, Box<dyn Error>>;

static SCRIPT_TEMPLATE: &str = r#"$shebang$
#SBATCH --job-name=$name$
#SBATCH --mem=$memory$
#SBATCH --time=$time$
#SBATCH --error=$error$
#SBATCH --output=$output$
$set$

$cmd$
"#;

static SCRIPT_TEMPLATE2: &str = r#"
#!/bin/bash

#SBATCH -e {log_dir}/{name}.%J.err
#SBATCH -o {log_dir}/{name}.%J.out
#SBATCH -J {name}

{header}

{bash_setup}

__script__"""
"#;


#[allow(clippy::too_many_arguments)]
pub fn make_submission_script(
    shebang: &str,
    set: &str,
    name: &str,
    memory: &str,
    time: &str,
    error: &str,
    output: &str,
    cmd: &str,
) -> String {
    let mut set_line = String::new();
    if !set.is_empty() {
        // let _ = write!(set_line, "set -{}", set);
        set_line = format!("set -{}", set);
    }
    SCRIPT_TEMPLATE
        .replace("$shebang$", shebang)
        .replace("$name$", name)
        .replace("$memory$", memory)
        .replace("$time$", time)
        .replace("$error$", error)
        .replace("$output$", output)
        .replace("$cmd$", cmd)
        .replace("$set$", &set_line)
}


// --------------------------------------------------
pub fn open(filename: &str) -> DynResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
// ---------------------------------------------------
pub fn find_matches(
    content: impl BufRead,
    pattern: &str,
    fallback_str: &str,
    mut writer: impl std::io::Write
) -> DynResult<()> {
    let mut has_match = false;
    for (line_num, line_result) in content.lines().enumerate() {
        let line = line_result?;
        if line.contains(&pattern) {
            writeln!(writer,"{} {}", &line_num, line);
            has_match = true;
        }
    }
    if !has_match {
        writeln!(writer,"{}", fallback_str);
    }
    Ok(())
}

pub fn print_command_str(
    mut writer: impl std::io::Write,
    command_str: &str,
) -> DynResult<()> {
    writeln!(writer,"{}", command_str);
    Ok(())
}

