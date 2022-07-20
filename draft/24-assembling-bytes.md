---
entry: 24
published: 2022-07-31
tag: v0.0.8-journey
---

# Assembling bytes

Our new assembler is almost done assembling. Over the last entries we learned how 
the program parses the assembly sourcecode and produces a list of parsed instructions. 
What we now need to do, is turn that into bytes.

## Parsed
Let us take a look at where we are. We have our sample program
[`hallo-stack.lass`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/pgm/hallo-stack.lass):

~~~
push_u8 123
push_u8 200
add
pop
fin
~~~

If we debug-print the `AsmPgm` after the parsing, it looks like this:

~~~
AsmPgm { 
    name: "pgm/hallo-stack.lass", 
    instructions: [
        AsmInstruction { line_number: 1, opcode:   2, oparg: [123], pos: 0 },
        AsmInstruction { line_number: 2, opcode:   2, oparg: [200], pos: 2 },
        AsmInstruction { line_number: 3, opcode:  16, oparg: [],    pos: 4 },
        AsmInstruction { line_number: 4, opcode:   1, oparg: [],    pos: 5 },
        AsmInstruction { line_number: 5, opcode: 255, oparg: [],    pos: 6 }
    ],
    line_number: 5, 
    text_pos: 7, 
    error: None
}
~~~

No error, that is nice. And we can see all five instructions parsed. We have a function that 
connects those bytes.

## Connect the bytes
~~~rust
/// Convert parsed assembly source to runnable program (or error report).
fn to_program(&self) -> Result<Pgm, AsmErrorReport> {
    if let Some(e) = &self.error {
        // Assembling failed:
        Err(AsmErrorReport{
            name: self.name.clone(),
            line: self.line_number,
            error: e.clone(),
        })
    } else {
        // Assembling succeeded, return a Pgm instance:
        let mut text: Vec<u8> = vec![];
        for i in &self.instructions {
            text.push(i.opcode);
            text.extend(&i.oparg);
        }
        Ok(Pgm{
            name: self.name.clone(),
            text,
        })
    }
}
~~~

The error part is straightforward. A small detail is the `clone()` call for name and error. We need 
to do that, because we cannot move ownership of those values (they must still exist in the `AsmPgm`
instance). And we cannot use references. There is no need to clone the line number; as an integer type 
it can simply be copied.

The success part isn't complex either. We create a Vector of bytes and push all bytes into it: for 
each instruction the opcode and the opargs (which there can be zero). We have our bytecode now!
Wrap it inside our new `Pgm` type, and we are done.

## Run the assembler
Let us see what our program looks like, assembled:
~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- pgm/hallo-stack.lass 

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas pgm/hallo-stack.lass`
Pgm { name: "pgm/hallo-stack.lass", text: [2, 123, 2, 200, 16, 1, 255] }
~~~

And how about our noisy program, 
[`noice.lass`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/pgm/noise.lass)?

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- pgm/noise.lass 

    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/lovas pgm/noise.lass`
Pgm { name: "pgm/noise.lass", text: [2, 123, 2, 200, 16, 1, 255] }

~~~

So it *does* produce the same bytecode for both. As we demanded.

## Running into errors
What happens, if our program has errors? Easy to find out, I included a broken 
program:
[`syntax-error.lass`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/pgm/syntax-error.lass)

~~~
push_u8 123
push_u8 300
add
pop
fin
~~~

Have you found the problem? Will the assembler?

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- pgm/syntax-error.lass 

    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/lovas pgm/syntax-error.lass`
Error: assembly failed in line 2 of program 'pgm/syntax-error.lass'

Caused by:
    InvalidArgument
~~~

It does find the error. Using the `parse_int` create already pays. And the error message 
really tells us, what is wrong and where. We get a lot of value for very few code we have written.

## Why AsmPgm?
There does not really seem to be a point of storing all that information inside `AsmPgm`. We could 
easily have created the bytecode directly. That would have been a lot easier. And if you have run 
the code yourself, you will have been bombarded with compiler warnings about unread fields.

We will be needing that information soon, and it was easiest to build it like this right away.
But let us just enjoy our new assembler for now.

## impl error::Error
Okay, before we leave for today, one more thing that you might have spotted. What's with that 
`impl` blocks?

~~~rust
impl Display for AsmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for AsmError {
}

impl Display for AsmErrorReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "assembly failed in line {} of program '{}'", self.line, self.name)
    }
}

impl error::Error for AsmErrorReport {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}
~~~

That is the price we have to pay when we want to use Rust magic. Rust's answer to writing 
generic code that can be applied to different types (that might not exist at the time 
of writing) are *traits*. A function can accept a trait as a type. If you implement that 
trait for your type, you can use that function. That is a very simplified introduction.

A trait defines specific functions you have to write for a type. That is what we do here.
We implement the trait `std::error::Error` for our `AsmError` and `AsmErrorReport`. To do so, 
we must also implement the trait `std::fmt::Display` (because the `Error` trait says so).

There is not much we do there. Types implementing the `Display` trait can be printed using 
`println!("{}", value)`. What the `println!` macro does is just calling that `fmt` method 
we define. The trait `Debug` does a similar thing, but for use with 
`println!("{:?}", value)`. We can use any value with those constructs that implements 
the `Display` trait (for `"{}"`) or the `Debug` trait (for `"{:?}"`).

The `Debug` trait we let the compiler implement (derive) for us. That is what the line 
`#[derive(Debug)]` does. And for our `Display` trait we are lazy and just use the 
function that was created by `#[derive(Debug)]`.

The `Error` trait lets you implement a `source()` method, that is used to get a nested 
Error inside your Error, that was its cause. Think of exception stacks, only that 
we do not have exceptions, of course. That is exactly what we want for `AsmErrorReport`; 
it is, after all, a wrapper for `AsmError`. `AsmError` on the other hand does not have 
a nested error, so we do not implement the `source()` method. The empty 
`impl error::Error for AsmError` block is still needed. If you remove it, the `Error` trait 
will not be implemented for `AsmError`.

Cool story, but why do we do all this? This is what enables us to use the magic of `anyhow` 
in our `lovas.rs`. We can use `AsmError` and `AsmErrorReport` (wrapped in an `Err()`) as return 
for our main function. It returns `anyhow::Result<()>`. And when there is an error returned 
by it, an error message is created and printed for us. With this we can easily create useful 
error messages in the error type itself, at the place where we understand, what errors 
exist and what they mean. And we need do it in that one place only. Every program that uses 
our library (as `lovas.rs` does) benefits from that without any extra work or even 
without knowing, error types can be returned by the library.
