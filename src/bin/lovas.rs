//! An experimental assembler for lovem
use std::time::Instant;
use clap::Parser;
use anyhow::{Context, Error, Result};
use lovem::{asm, Pgm, VM};

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

    #[clap(short, long, help = "Run the assembled program in lovem.")]
    run: bool,

   #[clap(long, help = "Enable tracing log when running lovem.")]
    trace: bool,
}

/// Executes a program in a freshly created lovem VM.
fn run(pgm: &Pgm, trace: bool) -> Result<()> {
    // Create our VM instance.
    let mut vm = VM::new(100);
    vm.trace = trace;
    let start = Instant::now();
    let outcome = vm.run(&pgm.text);
    let duration = start.elapsed();
    match outcome {
        Ok(_) => {
            // Execution successful, program terminated:
            eprintln!("Terminated.\nRuntime={:?}\npc={}, op_cnt={}, stack size={}, watermark={}",
                      duration,
                      vm.pc, vm.op_cnt, vm.stack.len(), vm.watermark
            );
            Ok(())
        },
        Err(e) => {
            // Runtime error. Error will be printed on return of main.
            eprintln!("Runtime error!\nRuntime={:?}\npc={}, op_cnt={}, stack size={}, watermark={}",
                      duration, vm.pc, vm.op_cnt, vm.stack.len(), vm.watermark);
            Err(Error::from(e))
        }
    }
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
            if args.run {
                // lovas was called with `--run`, so create a VM and execute program:
                run(&pgm, args.trace)
            } else {
                // Just debug print the program, we have no storage format, yet:
                println!("{:?}", pgm);
                Ok(())
            }
        },
        Err(e) => {
            // Something went wrong during assembly.
            // Convert the error report, so that `anyhow` can do its magic
            // and display some helpful error message:
            Err(Error::from(e))
        },
    }
}
