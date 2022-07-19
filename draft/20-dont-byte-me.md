---
entry: 20
published: 2022-07-31
tag: v0.0.7-journey
---

# Don't byte me!

__I have had it with these motherloving bytes in this motherloving bytecode!__

By now you should have come to a realisation: writing bytecode sucks! It wasn't fun to begin 
with, but now that we introduce jumps in our code, we need to count how many bytes the jump 
takes &ndash; and that with instructions that have different numbers of bytes as opargs. Encoding 
negative numbers in bytes is also no fun. And just think about it: if you change your program
(e.g. add a few instructions), you have to adjust those relative jumps! How horrible is that?
Can't someone else do it? Well, yeah, of course. We invented a machine that can do annoying and 
monotone tasks that require accuracy and that must be done over and over again. That machine 
is, of course, the computer.

Well, lucky us, that we know how to tell a computer what it should do. So let's write a program, 
that writes bytecode for us. I am not talking about compiling a programming language into our 
VM; at least not yet, not for a long time. But something that lets us write those instructions 
in a way that is at least a bit more human friendly.

Maybe you remember that I already tried to write some of the bytecode programs I showed you in 
a more readable way, like this:

~~~
push_u8 5
push_u8 7
push_u8 11
sub
mul
push_u8 13
push_u8 17
add
add
pop
fin
~~~

If that did remind you of something, that might be no coincidence.

## Assembler
The listing up there looks a bit like assembler code. And on the earlier draft of *lovem* I did 
already write a program that could translate those listings into bytecode. We will do that again, 
together. But this will take us some time (that is, multiple journal entries). We need to acquire 
some additional Rust skills for that. And there is so much to explain inside that assembler program 
itself.

Once again, I am making this up along the way. Yes, I have a plan, but I will just start to introduce 
syntax for the assembler, and it might not be ideal. That means, I might change it all again later.
As the VM itself, our assembler will be experimental. You are welcome to give me ideas for the syntax; 
we do have the comments now, unter each post, feel free to use them. There is the whole 
[GitHub discussions](https://github.com/kratenko/lovem/discussions) page as well. And you can still 
find me on Twitter. Find the link at the bottom of this page.

## Command line tool
The assembler will be a binary that you call with parameters. A typical command line tool, just 
like `gcc` or `rustc` are. So what we need to do, is to learn how one writes a command line tool 
in Rust. One that can read files, because I plan to write assembly programs in text files. And I 
have no desire to start parsing command line arguments myself. Neither do I want to write an 
introduction on writing command line tools in Rust. All this has been done. So I kindly direct you 
to an online book:

[Command Line Applications in Rust](https://rust-cli.github.io/book/index.html).

That is where I got what I will be using here. They use a crate called 
[clap](https://docs.rs/clap/latest/clap/), which seems to be the most used lib for building 
command line tools in Rust. It takes about 10 minutes to read. Finding out how to use the 
options of clap that I want took longer, but that will not be a thing for you, as I will just 
be using those options.

This is the first time we are using external crates in Rust. We need to add our dependencies 
to [Cargo.toml](https://github.com/kratenko/lovem/blob/v0.0.7-journey/Cargo.toml), before we can use them:

~~~toml
[dependencies]
clap = { version = "3.2.12", features = ["derive"] }
anyhow = "1.0.58"
~~~

## Introducing lovas
Now let us start with the assembler. We create a new binary that will become our assembler: 
[lovas.rs](https://github.com/kratenko/lovem/blob/v0.0.7-journey/src/bin/lovas.rs)

~~~rust
//! An experimental assembler for lovem
use clap::Parser;
use anyhow::{Context, Result};

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
~~~

As it happens with Rust, the code is very dense. I try to explain what I do inside the code using comments.
This does not look like it does too much. Yet it does.
You can call it using `cargo run --bin lovas`, as we learned earlier:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas`
error: The following required arguments were not provided:
    <SOURCE>

USAGE:
    lovas <SOURCE>

For more information try --help
~~~

That is already a lot! It finds out that you did not supply a required argument and tells you 
in a somewhat understandable error message. We did not write any of that. And it even directs 
you how to get help: add `--help` to your call.

Now if we use cargo to run our binary, we need to add an extra bit to the call, because we need to 
tell cargo where its own arguments end, end where the arguments to the called binary begin. This is 
done (as it is custom) by adding `--`, to indicate the end of cargo's arguments. So if we want to 
pass `--help` to lovas, we can do it like this:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- --help

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas --help`
lovas 
An experimental assembler for lovem, the Low Overhead Virtual Embedded Machine.

USAGE:
    lovas <SOURCE>

ARGS:
    <SOURCE>
            Path to assembler source file.

OPTIONS:
    -h, --help
            Print help information
~~~

How helpful! Also, now you can see why I added those two strings to our `Cli` struct; they show up 
in the help message.

## Run it
It looks like we need to give it a file to read, if we want the program to succeed and not 
exit with an error. I did write a little assembly program that we can use: 
[hallo-stack.lass](https://github.com/kratenko/lovem/blob/v0.0.7-journey/pgm/hallo-stack.lass). 
Our assembler will not so anything too useful with it, because we did 
not write an assembler, yet. It will simply print out the lines of the file, prefixed with 
the line number (the call to `.enumerate()` is what I use to count the lines, while iterating 
over them).

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- pgm/hallo-stack.lass

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas pgm/hallo-stack.lass`
   1: 'push_u8 123'
   2: 'push_u8 200'
   3: 'add'
   4: 'pop'
   5: 'fin'
~~~

Neat! I feel this is a lot for such a small program! It is also enough for this journal entry.
We will be working on `lovas` for a bit, now.

## Homework
Well - if you have not done so, read the book I linked. At least up until chapter 1.4, I guess, 
that is what we need for now.

And try to trigger some errors when calling `lovas`. What if the file you tell it to open does not 
exist? What if it cannot be read? Do you understand how those error messages propagate through the 
program and end up as a readable message in your console?
