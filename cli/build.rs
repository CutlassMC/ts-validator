#[path = "src/args.rs"]
mod args;

use anyhow::Result;
use clap::ValueEnum;
use clap_complete::aot::Shell;

const MISSING_OUTDIR: &str = "No env var OUT_DIR to write completions to.";

fn main() -> Result<()> {
    let out = std::env::var_os("OUT_DIR").ok_or_else(|| anyhow::anyhow!(MISSING_OUTDIR))?;

    Shell::value_variants().iter().for_each(|&shell| {
        let mut command: clap::Command = <args::Args as clap::CommandFactory>::command();

        let _ = clap_complete::generate_to(shell, &mut command, clap::crate_name!(), &out);
    });

    println!("cargo::rerun-if-changed=src/args.rs");

    Ok(())
}
