---
entry: 17
published: 2022-07-30
tag: v0.0.4-journey
---

# More operations

__The basic operation of the VM is working. Let us add a few more opcodes, so that we can do calculations.__

We have created a rust library that holds our virtual register machine. We can now add multiple executables to 
it, so that makes it easier, to write different programs and keep them (to mess around with the VM). We will 
add a few more opcodes to our repertoire, because only adding numbers is just plain boring.

I put some sort into what opcodes to introduce; but be advised, that none of them are final. Not only is the 
VM experimental and in a very early state, I introduce codes that I do not intend to keep on purpose. This is 
also a demonstration/introduction. So I add codes that are helpful at the time of writing, for experimenting. 
`FIN` is an example of a code, that will most likely be removed at some point. But for now it is nice to have 
a simple way to explicitly terminate the program. It gives some confidence, when we reach that point, that our 
program works as intended, and that we did not mess up the bytecode.

## Arithmetics
Baby steps. No rush here. We had *adding* as a first example. We will introduce *subtraction*, *multiplication*, 
*division*, and *modulo*. Sounds like not much, but we will run in some complications, anyways... Here is our 
addtion to `op.rs`.

~~~rust
/// opcode: Subtract top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const SUB: u8 = 0x11;

/// opcode: Multiply top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const MUL: u8 = 0x12;

/// opcode: Divide top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const DIV: u8 = 0x13;

/// opcode: Calculate modulo of top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const MOD: u8 = 0x14;
~~~

## The order of things

Simple enough those new codes, just copy and paste from `ADD`. But it turns out, subtraction is not as 
easy as addition. Here is the handling code we used for `ADD`:

~~~rust
op::ADD => {
    println!("  ADD");
    let a = self.pop()?;
    let b = self.pop()?;
    self.push(a + b)
},
~~~

Works. But if we copy and use that for `SUB`:

~~~rust
op::SUB => {
    println!("  SUB");
    let a = self.pop()?;
    let b = self.pop()?;
    self.push(a - b)
},
~~~

It turns out, that I messed up the order of the operands. That does not matter for addition, but subtraction 
is not commutative. So let's change that:

~~~rust
op::ADD => {
    println!("  ADD");
    let b = self.pop()?;
    let a = self.pop()?;
    self.push(a + b)
},
op::SUB => {
    println!("  SUB");
    let b = self.pop()?;
    let a = self.pop()?;
    self.push(a - b)
},
op::MUL => {
    println!("  MUL");
    let b = self.pop()?;
    let a = self.pop()?;
    self.push(a * b)
},
op::DIV => {
    println!("  DIV");
    let b = self.pop()?;
    let a = self.pop()?;
    self.push(a / b)
},
op::MOD => {
    println!("  MOD");
    let b = self.pop()?;
    let a = self.pop()?;
    self.push(a % b)
},
~~~

So, we learned something. I put the other operators there, as well. But this is too naive. 
You might already see the problem.

## Blowing up the school
As my math teacher liked to say: "... dann fliegt die Schule in die Luft!" &ndash; If we do that 
the school building will blow up. It is his way of dealing with the issue, that pupils are 
told "you must never divide by zero", but that they are never given an understandable reason 
for it. So just own it, and provide a completely absurde one.

What happens, is we keep it like this? Well, not much - until you write a program that divides 
by zero. Then, this will happen:

~~~
[...]
VM { stack: [4, 0], pc: 4, op_cnt: 2 }
Executing op 0x13
  DIV
thread 'main' panicked at 'attempt to divide by zero', src/vm.rs:142:31
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/panicking.rs:143:14
   2: core::panicking::panic
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/panicking.rs:48:5
   3: lovem::vm::VM::execute_op
             at ./src/vm.rs:142:31
   4: lovem::vm::VM::run
             at ./src/vm.rs:85:13
   5: modulo::main
             at ./src/bin/modulo.rs:10:11
   6: core::ops::function::FnOnce::call_once
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Process finished with exit code 101
~~~

Our program panics! I told you earlier, that this is not good behaviour. I introduced you to a lot 
of weird Rust stuff, just to avoid those. So, let us not re-introduce them now. So, what can we do instead?

Division by zero is a runtime error, for sure (at least in this numerical domain we are working with). But 
it should not be a runtime error in our virtual machine, it should be a runtime error in the program it is 
running. Luckily, we already have that mechanism in our VM. So let us add a new runtime error:

~~~rust
/// An error that happens during execution of a program inside the VM.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    UnknownOpcode(u8),
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
}
~~~

And adjust our opcode handlers:

~~~rust
op::DIV => {
    println!("  DIV");
    let b = self.pop()?;
    let a = self.pop()?;
    if b == 0 {
        Err(RuntimeError::DivisionByZero)
    } else {
        self.push(a / b)
    }
},
op::MOD => {
    println!("  MOD");
    let b = self.pop()?;
    let a = self.pop()?;
    if b == 0 {
        Err(RuntimeError::DivisionByZero)
    } else {
        self.push(a % b)
    }
},
~~~

We add a check for the `DIV` and `MOD` handlers (modulo is a division as well). If we run that 
program dividing by zero again, we now get this:

~~~
[...]
VM { stack: [4, 0], pc: 4, op_cnt: 2 }
Executing op 0x13
  DIV
Error during execution: DivisionByZero

Process finished with exit code 0
~~~

Yes, it still fails. But only the execution of the bytecode fails, not the execution of our 
virtual machine. You can now handle the problem inside your Rust program in a way that fits 
your needs. Much better. In the next post, we will be using our new instructions in a 
fancy way, that works well with a stack machine.

## Homework
Oh, not sure. Play around with it, I guess? As always. Feel free to write a calculation into 
a program and compare the results. It should work, unless I messed up again. You should have 
at least, at some point, write a program in bytecode yourself, so that you know how that feels.
