use anyhow::bail;
use clap::Parser;
use path_absolutize::Absolutize;
use posh_transient::alter_omp_json;

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    pub source: PathBuf,
    pub destination: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !args.destination.exists() {
        fs::create_dir_all(&args.destination)?;
    }

    let source_filestem = args
        .source
        .file_stem()
        .unwrap_or(OsStr::new("sometheme.omp"));
    let target_filename = format!(
        "{}.{}.{}",
        source_filestem.to_string_lossy(),
        "transient",
        args.source
            .extension()
            .map(|s| s.to_string_lossy())
            .unwrap_or("json".into())
    );

    let target_filepath = if args.destination.is_dir() {
        args.destination.join(target_filename)
    } else if args.destination.is_file() {
        args.destination
    } else {
        bail!("Not a valid destination, but how can it be???")
    };
    let target_filepath = target_filepath.absolutize()?;

    if target_filepath.is_file() {
        println!("{}", target_filepath.display());
        return Ok(());
    }

    let source_file = fs::File::open(&args.source)?;
    let mut json = serde_json::from_reader(source_file)?;

    alter_omp_json(&mut json);
    let json_transient = serde_json::to_vec_pretty(&json)?;

    fs::write(&target_filepath, json_transient)?;
    println!("{}", target_filepath.display());

    Ok(())
}
