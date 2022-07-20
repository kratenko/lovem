//! An experimental assembler for lovem
use clap::Parser;
use anyhow::{Context, Error, Result};
use lovem::asm;

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
    // Store the path to the program in a usable place:
    let name = args.source.as_path().display().to_string();
    // read complete source file into String:
    let content = std::fs::read_to_string(&args.source)
        .with_context(
            || format!("could not read file `{}`", &name)
        )?;
    // run the assembler:
    match asm::assemble(&name, &content) {
        Ok(pgm) => {
            // we succeeded and now have a program with bytecode:
            println!("{:?}", pgm);
            Ok(())
        },
        Err(e) => {
            // Something went wrong during assembly.
            // Convert the error report, so that `anyhow` can do its magic
            // and display some helpful error message:
            Err(Error::from(e))
        },
    }
}
