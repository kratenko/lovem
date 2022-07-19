---
entry: 19
published: 2022-07-31
tag: v0.0.6-journey
---

# Go ahead and jump!

__All our programs have been linear so far. Let's build the base for jumping around.__

In every program we have written so far, each instruction just advances the PC[^pc], until we reach the end.
That is very linear. We will now introduce a new opcode, that jumps to a different position in the 
program.

## A new opcode
How do we implement that? That is actually quite easy. Do you remember what I said about the PC? 
It is a special register, that always points to the instruction in the bytecode, that is 
executed next. So all our operation needs to do is modify the PC. We will give that opcode an 
oparg of two bytes, so we can tell it, where to jump to. Here is our new opcode in `op.rs`:

~~~rust
/// opcode: Relative jump.
///
/// pop: 0, push: 0
/// oparg: 2B, i16 relative jump
pub const GOTO: u8 = 0x20;
~~~

Now we have the dreaded `goto`. Don't be scared - on bytecode level, that is all well. We are not 
designing a high level language here, there will be gotos. But how do we fetch an `i16` from our 
bytecode? So far we can only fetch `u8`. So we add some more fetching:

## Fetch more than a byte
~~~rust
/// Reads the next byte from the bytecode, increase programm counter, and return byte.
fn fetch_u8(&mut self, pgm: &[u8]) -> Result<u8, RuntimeError> {
    if let Some(v) = pgm.get(self.pc) {
        self.pc += 1;
        Ok(*v)
    } else {
        Err(RuntimeError::EndOfProgram)
    }
}

/// Reads the next byte from the bytecode, increase programm counter, and return byte.
fn fetch_i8(&mut self, pgm: &[u8]) -> Result<i8, RuntimeError> {
    if let Some(v) = pgm.get(self.pc) {
        self.pc += 1;
        Ok(*v as i8)
    } else {
        Err(RuntimeError::EndOfProgram)
    }
}

/// Reads the next two bytes from the bytecode, increase programm counter by two, and return as i16.
fn fetch_i16(&mut self, pgm: &[u8]) -> Result<i16, RuntimeError> {
    let hi = self.fetch_i8(pgm)? as i16;
    let lo = self.fetch_u8(pgm)? as i16;
    Ok(hi << 8 | lo)
}
~~~

We already know `fn fetch_u8()`. `fn fetch_i8()` does almost the exact thing, only that it casts that 
byte from `u8` to `i8`. Simple enough. Casting in Rust has the beautiful syntax `<value> as <type>`.

So why do we need `i8`? Because we are building an `i16` from an `i8` and a `u8`. Just a bit of bit 
arithmetic. We can pass on potential `EndOfProgram` runtime errors easily with `?` and `Result`.
It allows us to write some short but still easy-to-read code, I think. So now we can fetch the 
value, we need for our jump. So let us write the handler for the opcode in `fn execute_op()`
of `vm.rs`.

## Goto
~~~rust
op::GOTO => {
    println!("  GOTO");
    let d = self.fetch_i16(pgm)?;
    self.pc += d;
    Ok(())
}
~~~

So, is that all? No, because we made a Rust-beginner-mistake. If we try and compile the code, we 
get an error: 

~~~
error[E0308]: mismatched types
   --> src/vm.rs:174:28
    |
174 |                 self.pc += d;
    |                            ^ expected `usize`, found `i16`
~~~

Yeah - Rust does not allow us to do calculations with different types of integers. We need 
to explicitly cast everything. Rust tries to avoid ambiguity, so no implicit conversions. 
And, to be honest, the compiler has a good point. We should care even more about that calculation; 
we want our VM to be robust. We change the handler to:

~~~rust
op::GOTO => {
    println!("  GOTO");
    let d = self.fetch_i16(pgm)?;
    self.relative_jump(pgm, d)
}
~~~

## Safe goto
And we add a new method (and we add a new RuntimeError):

~~~rust
/// Executes a checked relative jump; Runtime error, if jump leaves program. 
fn relative_jump(&mut self, pgm: &[u8], delta: i16) -> Result<(), RuntimeError> {
    println!("  Jump from {} by {}", self.pc, delta);
    if delta < 0 {
        let d = -delta as usize;
        if self.pc >= d {
            self.pc -= d;
            Ok(())
        } else {
            Err(RuntimeError::InvalidJump)
        }
    } else {
        let d = delta as usize;
        if self.pc + d < pgm.len() {
            self.pc += d;
            Ok(())
        } else {
            Err(RuntimeError::InvalidJump)
        }
    }
}
~~~

## Enter the loop
Now, let us write a [new program](https://github.com/kratenko/lovem/blob/v0.0.6-journey/src/bin/endless.rs)
that uses the `goto` opcode:

~~~rust
//! Create a VM and run a small bytecode program in it.
//!
//! This demonstrates the goto operation with an endless loop.
use lovem::{op, VM};

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::PUSH_U8, 123, op::GOTO, 0xff, 0xfb, op::FIN];
    // Create our VM instance.
    let mut vm = VM::new(100);
    // Execute the program in our VM:
    match vm.run(&pgm) {
        Ok(_) => {
            println!("Execution successful.")
        }
        Err(e) => {
            println!("Error during execution: {:?}", e);
        }
    }
}
~~~

I will write that bytecode down in a more readable format again:

~~~
push_u8 123
goto -5
fin
~~~

Only 3 instructions. And the `fin` will never be reached. 
That `0xff, 0xfb` after the `op::GOTO` is the 2 byte oparg: an `i16` 
with the value `-5`. But why `-5`? When the `goto` executed, we have read both oparg bytes, so 
the PC points to the `fin` at index 5. So adding `-5` to it will set the PC to `0`. The next 
executed instruction will be the `push_u8` once again. This is an endless loop. So will the 
program run forever? What do you think will happen? Let's try:

~~~
VM { stack: [], pc: 0, op_cnt: 0 }
Executing op 0x02
  PUSH_U8
  value: 123
VM { stack: [123], pc: 2, op_cnt: 1 }
Executing op 0x20
  GOTO
  Jump from 5 by -5
VM { stack: [123], pc: 0, op_cnt: 2 }
Executing op 0x02
  PUSH_U8
  value: 123
VM { stack: [123, 123], pc: 2, op_cnt: 3 }
Executing op 0x20
  GOTO

[...]

VM { stack: [123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123], pc: 0, op_cnt: 200 }
Executing op 0x02
  PUSH_U8
  value: 123
Error during execution: StackOverflow

Process finished with exit code 0

~~~

There is a `push_u8` operation in our endless loop. So it will fill our stack until it is full! The program 
hits a runtime error after 200 executed instructions. Great, now we tested that, too.

NOPE
=======

That is not very dynamic. We want to make decisions! We want to choose our path. What we want is 
*branching*. We will introduce a new opcode, that will decide, which branch the execution of 
our program will take, based on a value during runtime. If this sounds unfamiliar to you, let me 
tell you, what statement we want to introduce: it is the *if* statement.

So, how does that work? As mentioned, normally the PC is incremented on each byte we fetch from the 
bytecode. And the PC always points to the next instruction, that will be executed. So if we want to 
change the path of execution, what we have to do is change the value of the PC.

An operation, that simply changes the PC statically, would be a *GOTO* statement. But there is no 
branching involved in that, the path that will be executed is always clear. The *if* statement on 
the other hand only alters the PC, if a certain condition is met.

## A new opcode

~~~rust
/// opcode: Branch if top value is equal to zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFEQ: u8 = 0x20;
~~~

Our new operation pops only one value. So what does it get compared to? That's easy: zero. If you 
need to compare two values to each other, just subtract them instead, and then you can compare with zero.
That gives the same result.

And what kind of oparg does this operation take? A signed integer. That is the value that should be 
added to the PC, if our condition is met. This will result in a relative jump.

## Homework
Same as always. Write some bytecode. Try some jumping around. Run into troubles! You can write a 
program, that has a `fin` in the middle, but executes code that lies behind that instruction.

[^pc]: PC: the Program Counter, a special register that points to the next instruction to be executed.
