---
entry: 21
published: 2022-07-30
tag: v0.0.8-journey
---

# Assemble!

__We introduce an API for assembly to our lovem library.__

Last time, we built the frame of a command line program, that will become our new assembler, `lovas`.
It is time that we give that program the power to assemble.

## Calling the real assembler
`lovas.rs` is just the executable wrapper around the actual assembler, that will live inside the 
library. All `lovas.rs` does, is supply the command line interface. And that CLI-part does not 
belong in a library function. We got it nicely separated. And programs using the library 
can assemble source to bytecode themselves, without calling an external binary.

We alter [`lovas.rs`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/src/bin/lovas.rs) a bit. 
The part that just printed out the source lines is gone. We replace it with a call to a new 
library function, that can transfer assembly code into bytecode:

~~~rust
fn main() -> Result<()> {
    ... the same as before ...
    
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
~~~

The important part is the call to `asm::assemble(&name, &constent)`. We created a new 
module `asm` inside our lib. It exposes only a single function `assemble` and a few types for 
error handling. There will be a lot to unpack inside that module.

The good news for us is: we do not need to restrain ourselves as much as we do in the VM itself.
Resource usage is not really an issue here, because the assembler is not meant to run in 
a restricted environment. The idea of *lovem* is, that you write your programs elsewhere, outside 
the restricted environment, and only run the compiled bytecode in the VM on the restricted device.
And since the scope handled by the assembler will still be defined by that restricted device, we 
expect to only write relatively small and simple programs. With modern computers used for 
assembling, we can use as much memory as we want.

Oh, by the way... Yeah, I seem to stick to these short, cryptic names for the parts of *lovem*.
`VM`, `Pgm`, `op`, `asm` - I kinda like it that way, and it goes well with the register names
etc. That feels right for something as low-lever as a VM. And I give my best to always 
document those things properly, so that your IDE of choice will always show you, what 
each thing is.

## ASM
I wrote a very basic assembler inside 
[`asm.rs`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/src/asm.rs), and it is 
already over 250 lines long. Quite a lot to unpack. As before, I try to explain as much 
as possible inside the source code itself, using comments. This makes it easier to follow, 
and you can even do so inside the source in the repo, without reading this blog.

There are four types that I introduce inside the mod:

~~~rust
/// Errors that can happen during assembly.
#[derive(Debug, Clone)]
pub enum AsmError {
    InvalidLine,
    UnknownInstruction(String),
    UnexpectedArgument,
    MissingArgument,
    InvalidArgument,
}

/// Report of failed assembly attempt.
///
/// Wraps the error that occurred during assembly and supplied information where it did.
#[derive(Debug)]
pub struct AsmErrorReport {
    /// Name of the program that failed to assemble.
    name: String,
    /// Line the error occurred during assembly.
    line: usize,
    /// Error that occurred.
    error: AsmError,
}

/// A single instruction parsed from the line of an assembly program.
#[derive(Debug)]
struct AsmInstruction {
    /// Number of line the instruction was read from.
    ///
    /// The number of the line the instruction was taken from, most likely
    /// from a source file. Line counting starts at 1.
    line_number: usize,
    /// Opcode defining which operation is to be executed.
    opcode: u8,
    /// Arguments used for execution of the operation.
    ///
    /// Zero or more bytes.
    oparg: Vec<u8>,
    /// Position inside bytecode (starting at 0).
    ///
    /// Number of bytes that come before this instruction in the program.
    pos: usize,
}

/// A assembler program during parsing/assembling.
#[derive(Debug)]
struct AsmPgm {
    /// Name of the program (just a string supplied by caller).
    name: String,
    /// Vector of parsed assembler instructions, in the order they are in the source file.
    instructions: Vec<AsmInstruction>,
    /// Current line number during parsing.
    ///
    /// Used for error reporting.
    line_number: usize,
    /// Current position inside bytecode during parsing.
    ///
    /// Used to calculate the exact position an instruction will be in the bytecode.
    text_pos: usize,
    /// The error that happened during parsing/assembling, if any.
    error: Option<AsmError>,
}
~~~

`AsmError` is easy enough to understand. We used the same idea for the `RuntimeError` inside the VM.
When we run into an Error while trying to assemble the program, we return `Err<AsmError>` instead 
of `Ok(())`, so that we can propagate what happened back to the caller.
The nice thing is, that with speaking names for the enum values, and with the occasional embedded 
value (as in `UnknownInstruction(String)`), the debug representation of the `AsmError` alone is 
enough to make the user understand what error was detected.

`AsmErrorReport` is a little wrapper we use to add the information *where* we ran into an error.
`InvalidArgument` is nice hint how to fix your program - but if that program is 2000 lines long, 
then good luck. When you know the `InvalidArgument` happened in line 1337, then you will find it 
much faster. Especially in an assembly language, that has never more than one single instruction 
per line.

`AsmInstruction` is used to represent a single instruction inside a program. So each instance 
of this type will be linked to a specific line in the source file. If you don't remember, what 
counts as an instruction in *lovem* (at least at the time of writing), let me repeat: 
an *instruction* consists of exactly one *operation* that is to be executed, which is identified by 
its *opcode* (which is a number from `0x00` to `0xff` stored in a single byte). Each instruction 
has zero or more bytes used as an argument, defining how the operation is to be executed. This 
argument is called *oparg*. We will also store the number of the line we found our instruction 
inside the source code, and the position inside the bytecode where the instruction will be.

`AsmPgm` will represent the complete program during the assembly process. We will collect the 
instructions we parse from the source in there in a Vector. And we will hold the progress during 
parsing/assembling. This is not the type that will be returned to the caller, it is only used 
internally (as you can guess by the fact that it is not defined `pub`).

## Where does the program come from?
The only function the mod exports it `assemble`:

~~~rust
/// Parse assembly source code and turn it into a runnable program (or create report).
pub fn assemble(name: &str, content: &str) -> Result<Pgm, AsmErrorReport> {
    let asm_pgm = AsmPgm::parse(name, content);
    asm_pgm.to_program()
}
~~~

It will return an `AsmErrorReport`, if anything goes wrong and the assembling fails. If the 
assembler succeeds, it returns an instance of `Pgm`. Now where does that come from? Our VM 
takes programs in form of a `&[u8]`. That will be changed soon, and then it will run programs 
from a special type `Pgm` that might have a bit more than just bytecode. I added another 
new module to the library: 
[`pgm.rs`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/src/pgm.rs). 
That one is tiny and only holds the new `struct Pgm` &ndash; which itself is basic. But we 
have a type that holds a program, now. I believe that will be beneficial to us later.

~~~rust
/// Holds a program to be executed in VM.
#[derive(Debug)]
pub struct Pgm {
    /// Some name identifying the program.
    pub name: String,
    /// Bytecode holding the programs instructions.
    pub text: Vec<u8>,
}
~~~

What is it, that the assembler does, to create such a `Pgm`. We will start to go through that 
in the next entry. This has been enough for today.
