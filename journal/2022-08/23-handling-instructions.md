---
entry: 23
published: 2022-08-08
tag: v0.0.8-journey
---

# Handling instructions

We took care of all the dirty work inside the assembler during the previous posts. We now have 
a cleanly parsed instruction with an optional argument that we can evaluate. Let us dive 
into `parse_instruction()`:

~~~rust
/// Handles a single instruction of opcode an optional oparg parsed from Assembly file.
fn parse_instruction(&mut self, opname: &str, oparg: Option<&str>) -> Result<(), AsmError> {
    match opname {
        "nop" => self.parse_a0_instruction(op::NOP, oparg),
        "fin" => self.parse_a0_instruction(op::FIN, oparg),
        "pop" => self.parse_a0_instruction(op::POP, oparg),
        "add" => self.parse_a0_instruction(op::ADD, oparg),
        "sub" => self.parse_a0_instruction(op::SUB, oparg),
        "mul" => self.parse_a0_instruction(op::MUL, oparg),
        "div" => self.parse_a0_instruction(op::DIV, oparg),
        "mod" => self.parse_a0_instruction(op::MOD, oparg),
        "push_u8" => {
            let oparg = oparg.ok_or(AsmError::MissingArgument)?;
            let v = parse_int::parse::<u8>(oparg).or(Err(AsmError::InvalidArgument))?;
            self.push_a1_instruction(op::PUSH_U8, v)
        },
        "goto" => {
            let oparg = oparg.ok_or(AsmError::MissingArgument)?;
            let v = parse_int::parse::<i16>(oparg).or(Err(AsmError::InvalidArgument))?;
            let a = v.to_be_bytes();
            self.push_a2_instruction(op::GOTO, a[0], a[1])
        },
        _ => Err(AsmError::UnknownInstruction(String::from(opname)))
    }
}
~~~

That is a surprisingly simple function. It receives two parameters. `opname` is a `&str` that 
holds the `opname` of the instruction. `oparg` is either `None`, if there was no argument in 
the instruction, or it holds a none-empty string that holds whatever argument was present in the 
instruction.

The function only consists of a long `match`, that directly matches the `opname` against our 
known opnames. If there is no match, it returns a helpful error that even contains the 
unknown opname that was found.

The explicit branches look a bit weirder. That is because I do not like to repeat myself 
when writing code. And Rust tends to allow some very dense source code. 

## Different kind of instructions
I decided to group by instructions into three categories. They are grouped by the number of bytes 
an instruction uses as argument. An `a0` instruction has zero bytes of oparg, `a1` has one byte, 
and `a2` has two bytes. 

### a0
Most of our operations do not allow any argument at all. We want to make 
sure that there is none given in the instruction. And the only difference in handling those 
instructions inside the assembler is the byte that will be written to the bytecode. We can 
handle all of those with the same function: `parse_a0_instruction()`:

~~~rust
/// Helper that parses an instruction with no oparg and pushes it.
fn parse_a0_instruction(&mut self, opcode: u8, oparg: Option<&str>) -> Result<(), AsmError> {
    if oparg.is_some() {
        Err(AsmError::UnexpectedArgument)
    } else {
        self.push_a0_instruction(opcode)
    }
}
~~~

If we did get an argument, we fail, since that is not allowed. And then we push a very basic 
instruction to the back of our program. We have helper functions to do that:

~~~rust
/// Adds a single instruction to the end of the AsmProgram.
fn push_instruction(&mut self, i: AsmInstruction) -> Result<(), AsmError> {
    self.text_pos += i.size();
    self.instructions.push(i);
    Ok(())
}

/// Helper that creates an instruction with 0 bytes of oparg and pushes it.
fn push_a0_instruction(&mut self, opcode: u8) -> Result<(), AsmError> {
    let i = AsmInstruction{
        line_number: self.line_number,
        opcode,
        oparg: vec![],
        pos: self.text_pos,
    };
    self.push_instruction(i)
}
~~~

We create a new instruction instance and add it. We also track the position of every instruction 
in the bytecode, that is why we update the programs current position in the bytecode for 
every instruction we add (stored in `text_pos`).

There is nothing we do with that information, yet. But we will need that information later.

### a1: `push_u8`
We only have one operation that needs a single byte of oparg, and that is `push_u8`. We use that 
operation to push values on the stack, taken directly from the bytecode. `u8` is the only type 
supported at the moment. That is not even a hard restriction; you can easily get any `i64` value 
to the stack by using basic arithmetics, and we have those.

Parsing numbers is no fun. It is hard. So we let someone else do it for us.
The crate we are using is called [`parse_int`](https://crates.io/crates/parse_int). 
Go take a look at what it can do. It allows us to enter numbers easily in hexadecimal, octal, or 
binary notation. That is a really handy feature in source code! Thanks, Rust community!
So how are we parsing `push_u8`?

~~~rust
"push_u8" => {
    let oparg = oparg.ok_or(AsmError::MissingArgument)?;
    let v = parse_int::parse::<u8>(oparg).or(Err(AsmError::InvalidArgument))?;
    self.push_a1_instruction(op::PUSH_U8, v)
},
~~~

First we make sure that we have an argument. If not, we fail. We can again use our handy `?` syntax.
Then we try to parse it into a `u8`, using `parse_int`. The syntax for that call takes some 
getting used to - I'm still waiting for me to getting used to it. But if it works, we now have 
a valid `u8`. If it fails to parse, we quickly return with that failure information.
If all goes well we will reach the third line, that calls our helper for adding a1 instructions. 
There is no big surprise in what that function does:

~~~rust
/// Helper that creates an instruction with 1 byte of oparg and pushes it.
fn push_a1_instruction(&mut self, opcode: u8, a0: u8) -> Result<(), AsmError> {
    let i = AsmInstruction{
        line_number: self.line_number,
        opcode,
        oparg: vec![a0],
        pos: self.text_pos,
    };
    self.push_instruction(i)
}
~~~

An interesting detail is, that `push_instruction()` returns a `Result`, even though it 
can never fail! It always returns `Ok(())`. And if you look at `push_a2_instruction()`, you 
will now see that it also will always return `Ok(())`. We do be bother? Take a look at 
the handler for `push_u8` again, in context of the complete function `parse_instruction()`.
That function returns a `Result`, and it can return `Err(...)`. Because `push_a1_instruction()` 
has the same return value of `Result`, the calls integrate nicely with the layout of the 
complete function inside the `match`. For me, it gives the code a clean compactness.

### a2: `goto`
There is one more branch to look at:

~~~rust
"goto" => {
    let oparg = oparg.ok_or(AsmError::MissingArgument)?;
    let v = parse_int::parse::<i16>(oparg).or(Err(AsmError::InvalidArgument))?;
    let a = v.to_be_bytes();
    self.push_a2_instruction(op::GOTO, a[0], a[1])
},
~~~

This time we use `parse_int` to read a `i16`. Whether you like the `::<i16>` syntax or not, at least 
you can see what it is for. We need to unpack the two bytes of the `i16` after parsing, so that 
we can store the bytes correctly in the bytecode. `to_be_bytes()` gives us an array (of size 2) 
that holds the bytes in big endian byte order. `to_le_bytes()` is the little endian counterpart. 
I generally prefer big endian, when I can. And if you remember how we read the bytes in the VM, 
you can see that we are already using big endian there.

There is nothing new in the `push_a2_instruction()` function, only one additional byte.

~~~rust
/// Helper that creates an instruction with 1 byte of oparg and pushes it.
fn push_a2_instruction(&mut self, opcode: u8, a0: u8, a1: u8) -> Result<(), AsmError> {
    let i = AsmInstruction{
        line_number: self.line_number,
        opcode,
        oparg: vec![a0, a1],
        pos: self.text_pos,
    };
    self.push_instruction(i)
}
~~~

## Parsing completed
We have now parsed the complete program source into the `AsmPgm` structure. Or we have failed to 
do so, in which case there is an Error stored in `AsmPgm`. Either way, you have now seen all the 
code that does the parsing. Next journal entry will finally produce the bytecode we are longing for.
