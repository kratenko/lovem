//! An experimental assembler for lovem
use clap::Parser;
use anyhow::{Context, Result};

// You can find an introduction to clap here:
// https://rust-cli.github.io/book/index.html

/// Struct used to declare the command line tool behaviour using clap.
///
/// This defines the arguments and options the tool provides. It is also used to
/// generate the instructions you get when calling it with `--help`.
#[derive(Parser, Debug)]
#[clap(name = "lovas",
long_about = "An experimental assembler for lovem, the Low Overhead Virtual Embedded Machine.",
)]
struct Cli {
    #[clap(parse(from_os_str), help = "Path to assembler source file.")]
    source: std::path::PathBuf,
}

fn main() -> Result<()> {
    // read, validate, and evaluate command line parameters:
    let args = Cli::parse();
    // read complete source file into String:
    let content = std::fs::read_to_string(&args.source)
        .with_context(
            || format!("could not read file `{}`", args.source.as_path().display().to_string())
        )?;
    // For now, just print our all the lines in the file:
    for (n, line) in content.lines().enumerate() {
        println!("{:4}: '{}'", n + 1, line);
    }
    // We succeeded in our work, so return Ok() as a Result:
    Ok(())
}
