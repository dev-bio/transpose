use std::{
    
    path::{PathBuf}, 
    fs::{File},
};


use anyhow::{
    
    Context,
    Result, 
};

use serde_json::{Value};
use serde::{Serialize};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Transpose {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn read_input(path: PathBuf) -> Result<Value> {
    let input_file = File::open(path.as_path()).context(format!("failed to open input file: {path:?}"))?;
    let input_extension = path.extension().context("missing input extension")?
        .to_str().context("malformed input extension")?;

    match input_extension {
        "yaml" | "yml" => {
            Ok(serde_yaml::from_reader(input_file).context(format!("failed to parse: {path:?}"))?)
        }
        "json" => {
            Ok(serde_json::from_reader(input_file).context(format!("failed to parse: {path:?}"))?)
        }
        _ => anyhow::bail!("unsupported input: {path:?}")
    }
}

fn write_output(path: PathBuf, ref data: impl Serialize) -> Result<()> {
    let output_file = File::create(path.as_path()).context(format!("failed to create output file: {path:?}"))?;
    let output_extension = path.extension().context("missing output extension")?
        .to_str().context("malformed output extension")?;

    match output_extension {
        "yaml" | "yml" => {
            Ok(serde_yaml::to_writer(output_file, data)
                .context(format!("failed to write: {path:?}"))?)
        }
        "json" => {
            Ok(serde_json::to_writer_pretty(output_file, data)
            .context(format!("failed to write: {path:?}"))?)
        }
        _ => anyhow::bail!("unsupported output: {path:?}")
    }
}

fn main() -> Result<()> {
    let Transpose { input, output } = Transpose::parse();
    Ok(write_output(output, read_input(input)?)?)
}
