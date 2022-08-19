---
entry: 27
published: 2022-08-19
tag: v0.0.11-journey
---

# What if?

__Choose your path.__

Our assembler gives us a lot of convenience for testing features of our VM. So let us start doing 
interesting stuff with it. We do have support for jumps already, but as it is now, save of an 
endless loop, there is absolutely no reason to do it, yet. All our programs run their predetermined 
way. If you look again at `label.lva`, you can see that none of those `goto`s introduce any dynamic.
We could just ditch them and reorder the rest. It would do the same, only more efficient. They simple 
tangle up our linear code, without removing its linearity.

Today we will introduce branches to our VM. A branch is a point in a program from which there are 
multiple possible paths to take. Two paths, normally. Which of those paths is takes is decided at 
runtime by looking at the state of the program. For us that means that we look at the value on top 
of the stack. How does it work?

## Conditional jump
We already introduced the `goto` operation. What we will add now, works exactly the same way, but 
only *if* a certain condition is met. And, yes, we will call that operation *if*. But if what? 
How about *if equal*? 

So we get the new opname `ifeq`, that pops a value from the stack and only executes its jump when 
that value is equal. Equal to what, you want to know? How about if it is equal to zero. If you want 
to compare it to a different number, it is easy to subtract that number from your value before you 
compare it to zero, and you achieve what you need.

## New operations
We will introduce multiple if-operations. Six, to be precise.

~~~ rust title="src/op.rs" linenums="63"
/// opcode: Conditional relative jump (branch) on pop == zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFEQ: u8 = 0x21;

/// opcode: Conditional relative jump (branch) on pop != zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFNE: u8 = 0x22;

/// opcode: Conditional relative jump (branch) on pop < zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFLT: u8 = 0x23;

/// opcode: Conditional relative jump (branch) on pop <= zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFLE: u8 = 0x24;

/// opcode: Conditional relative jump (branch) on pop > zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFGT: u8 = 0x25;

/// opcode: Conditional relative jump (branch) on pop >= zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFGE: u8 = 0x26;
~~~

And we add another operation, while we add it: `dup`

~~~ rust title="src/op.rs" linenums="21"
/// opcode: Pop value from stack and push it back, twice.
///
/// pop: 1, push: 2
/// oparg: 0
pub const DUP: u8 = 0x03;
~~~

This one simply duplicates the value on top of the stack, so that there will be another copy of it 
on top of it. We will use that often when testing values with an `if`, if we still need the value after 
testing it. The `if` will consume the top most value.

## Extending the assembler

We add the parsing handlers for our new instructions:

~~~ rust title="src/asm.rs" linenums="235" hl_lines="6 18 19 20 21 22 23"
fn parse_instruction(&mut self, opname: &str, oparg: Option<&str>) -> Result<(), AsmError> {
    match opname {
        "nop" => self.parse_a0_instruction(op::NOP, oparg),
        "fin" => self.parse_a0_instruction(op::FIN, oparg),
        "pop" => self.parse_a0_instruction(op::POP, oparg),
        "dup" => self.parse_a0_instruction(op::DUP, oparg),
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
        "goto" => self.parse_label_instruction(op::GOTO, oparg),
        "ifeq" => self.parse_label_instruction(op::IFEQ, oparg),
        "ifne" => self.parse_label_instruction(op::IFNE, oparg),
        "iflt" => self.parse_label_instruction(op::IFLT, oparg),
        "ifle" => self.parse_label_instruction(op::IFLE, oparg),
        "ifgt" => self.parse_label_instruction(op::IFGT, oparg),
        "ifge" => self.parse_label_instruction(op::IFGE, oparg),
        _ => Err(AsmError::UnknownInstruction(String::from(opname)))
    }
}
~~~

And that is all we need to change on our assembler. The way we have written it, it is easy to introduce 
new operations, when they share the same syntax in assembly and in bytecode as existing ones.

## Adjust the VM
First, we add the handler for the `dup`. Just pop a value and push it back, twice. Easy.
~~~ rust title="src/vm.rs" linenums="175"
op::DUP => {
    let v = self.pop()?;
    self.push(v)?;
    self.push(v)?;
    Ok(())
},
~~~

And now, the `if*`-handlers. They are similar to the `goto`-handler, just with an `if` added. 

~~~ rust title="src/vm.rs" linenums="218"
op::GOTO => {
    let d = self.fetch_i16(pgm)?;
    self.relative_jump(pgm, d)
},
op::IFEQ => {
    let d = self.fetch_i16(pgm)?;
    let v = self.pop()?;
    if v == 0 {
        self.relative_jump(pgm, d)
    } else {
        Ok(())
    }
},
op::IFNE => {
    let d = self.fetch_i16(pgm)?;
    let v = self.pop()?;
    if v != 0 {
        self.relative_jump(pgm, d)
    } else {
        Ok(())
    }
},
op::IFLT => {
    let d = self.fetch_i16(pgm)?;
    let v = self.pop()?;
    if v < 0 {
        self.relative_jump(pgm, d)
    } else {
        Ok(())
    }
},
op::IFLE => {
    let d = self.fetch_i16(pgm)?;
    let v = self.pop()?;
    if v <= 0 {
        self.relative_jump(pgm, d)
    } else {
        Ok(())
    }
},
op::IFGT => {
    let d = self.fetch_i16(pgm)?;
    let v = self.pop()?;
    if v > 0 {
        self.relative_jump(pgm, d)
    } else {
        Ok(())
    }
},
op::IFGE => {
    let d = self.fetch_i16(pgm)?;
    let v = self.pop()?;
    if v >= 0 {
        self.relative_jump(pgm, d)
    } else {
        Ok(())
    }
},
~~~

And that is all the code we have to change. Our VM can now execute conditional jumps. Now we can do 
some serious programming!

## A for-loop
Can't wait to use an *if* in program:

~~~ title="pgm/loop.lva" linenums="1"
# Demonstrate the conditional jump (a branch)
# The program has a loop that it executes thrice, before it terminates.
  push_u8 3
loop:
  push_u8 1
  sub
  dup
  ifgt loop
  pop
  fin
~~~

And execute it:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/loop.lva --print --trace
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas -r pgm/loop.lva --print --trace`
Pgm { name: "pgm/loop.lva", text: [2, 3, 2, 1, 17, 3, 37, 255, 249, 1, 255] }
VM { stack: [], pc: 0, op_cnt: 0, trace: true, watermark: 0 }
Executing op 0x02
VM { stack: [3], pc: 2, op_cnt: 1, trace: true, watermark: 1 }
Executing op 0x02
VM { stack: [3, 1], pc: 4, op_cnt: 2, trace: true, watermark: 2 }
Executing op 0x11
VM { stack: [2], pc: 5, op_cnt: 3, trace: true, watermark: 2 }
Executing op 0x03
VM { stack: [2, 2], pc: 6, op_cnt: 4, trace: true, watermark: 2 }
Executing op 0x25
  Jump from 9 by -7
VM { stack: [2], pc: 2, op_cnt: 5, trace: true, watermark: 2 }
Executing op 0x02
VM { stack: [2, 1], pc: 4, op_cnt: 6, trace: true, watermark: 2 }
Executing op 0x11
VM { stack: [1], pc: 5, op_cnt: 7, trace: true, watermark: 2 }
Executing op 0x03
VM { stack: [1, 1], pc: 6, op_cnt: 8, trace: true, watermark: 2 }
Executing op 0x25
  Jump from 9 by -7
VM { stack: [1], pc: 2, op_cnt: 9, trace: true, watermark: 2 }
Executing op 0x02
VM { stack: [1, 1], pc: 4, op_cnt: 10, trace: true, watermark: 2 }
Executing op 0x11
VM { stack: [0], pc: 5, op_cnt: 11, trace: true, watermark: 2 }
Executing op 0x03
VM { stack: [0, 0], pc: 6, op_cnt: 12, trace: true, watermark: 2 }
Executing op 0x25
VM { stack: [0], pc: 9, op_cnt: 13, trace: true, watermark: 2 }
Executing op 0x01
VM { stack: [], pc: 10, op_cnt: 14, trace: true, watermark: 2 }
Terminated!
VM { stack: [], pc: 11, op_cnt: 15, trace: true, watermark: 2 }
Terminated.
Runtime=100.972µs
op_cnt=15, pc=11, stack-depth=0, watermark=2
~~~

Nice! This is basically a for-loop. Granted, it does not do anything but loop, but you can see 
how the program counts down from `3` to `0` and after the third time it reaches line 8, 
it stops jumping back to `loop:` and advances to the end.

We can increase the number in line 3, and the number of runs increase with it. If we change it to `200`, 
we get this (I ditched the `--trace` for this).

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/loop.lva --print
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas -r pgm/loop.lva --print`
Pgm { name: "pgm/loop.lva", text: [2, 200, 2, 1, 17, 3, 37, 255, 249, 1, 255] }
Terminated.
Runtime=128.709µs
op_cnt=803, pc=11, stack-depth=0, watermark=2
~~~

More than 800 operations with only 10 lines of code. Shall we cranc it up to a million?

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/loop.lva --print
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas -r pgm/loop.lva --print`
Pgm { name: "pgm/loop.lva", text: [2, 100, 2, 100, 18, 2, 100, 18, 2, 1, 17, 3, 37, 255, 249, 1, 255] }
Terminated.
Runtime=564.184652ms
op_cnt=4000007, pc=17, stack-depth=0, watermark=2
~~~

Takes about have a second to execute, over 4000000 operations where executed. And the stack 
never held more than 2 values, as you can see by the watermark. We are programming!

## Homework
Wait a second! Our only way of getting values on the stack is `push_u8`. That can only push 
a `u8`, so only values `0` - `255`. How did I push that `1000000` there?
