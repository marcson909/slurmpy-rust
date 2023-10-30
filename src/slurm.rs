use std::collections::HashMap;
use std::{default, todo};

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


pub struct Slurm {
    name: String,
    slurm_kwargs: HashMap<String, String>,
    template: String,
    date_in_name: bool,
    scripts_dir: String,
    log_dir: String,
    bash_strict: bool,
}

impl Default for Slurm {
    fn default() -> Self {
        Self {
            name: "UPRD".into(),
            slurm_kwargs: HashMap::new(),
            template: SCRIPT_TEMPLATE.into(),
            date_in_name: true,
            scripts_dir: "slurm-scripts".into(),
            log_dir: "logs".into(),
            bash_strict: true
        }
    }
}

impl Slurm {
    fn new(
        name: String,
        // slurm_kwargs: HashMap<String, String>,
        // template: String,
    ) -> Slurm {
            Slurm {
            name: name,
            ..Default::default()
            // slurm_kwargs: HashMap::new(),
            // template: (),
            // date_in_name: (),
            // scripts_dir: (),
            // log_dir: (),
            // bash_strict: ()
        }
    }
}

// #[derive(Hash, Eq, PartialEq, Debug)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     /// Creates a new Viking.
//     fn new(name: &str, country: &str) -> Viking {
//         Viking { name: name.to_string(), country: country.to_string() }
//     }
// }

// Use a HashMap to store the vikings' health points.
// let vikings = HashMap::from([
//     (Viking::new("Einar", "Norway"), 25),
//     (Viking::new("Olaf", "Denmark"), 24),
//     (Viking::new("Harald", "Iceland"), 12),
// ]);