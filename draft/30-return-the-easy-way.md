---
entry: 30
published: 2022-07-31
tag: v0.0.14-journey
---

# Return the easy way

__A `goto` is fine and all - but we want to `return`!__

We have the ability to jump around in our program. But it is only `goto`. There is no way to add a 
`return` yet. You would have to `goto` back - and how would you know where to? We will implement 
one way to do that - and then talk about it a bit.

## The Frame Stack
Many machines (virtual or not, stack or register) include a kind of call stack. This is used to document 
your calling of functions. When you do a call, there is a new *frame* generated, that is used to represent 
the context of that called function; it contains information on where to return, when you leave the 
function. It is also used for realising function parameters, return values, and local variables. We will 
try and work our way towards it. To achieve this easily, we will introduce a second stack to our 
state machine, that we call *frame stack*.

## Call and Return
For the moment, all we want to introduce is the ability to return to the place where we started our 
call from. So what do we need to store in our *frame* is the address inside our bytecode where 
we came from. That is the value of the PC on the time of the call. So we add this to our VM:

~~~ rust title="src/vm.rs" linenums="36"
/// Frame Stack, holding frames for nested execution and returning.
pub fstack: Vec<usize>,
~~~

Just a new stack that holds `usize` which is the type of PC. We use a `Vec` as stack. 

We add two new opcodes:

~~~ rust title="src/op.rs" linenums="117"
/// opcode: Save return position and jump.
///
/// pop: 0, push: 0
/// oparg: 2B, i16 relative jump
pub const CALL: u8 = 0x27;

/// opcode: Return from `CALL`.
///
/// pop: 0, push: 0
/// oparg: 0B
pub const RET: u8 = 0x28;
~~~

Parsing them in `lovas` is simple enough:

~~~ rust title="src/asm.rs" linenums="300"
"ret" => self.parse_a0_instruction(op::RET, oparg),
"call" => self.parse_label_instruction(op::CALL, oparg),
~~~

And the handlers in our VM:

~~~ rust title="src/vm.rs" linenums="327"
op::CALL => {
    let d = self.fetch_i16(pgm)?;
    self.fstack.push(self.pc);
    self.relative_jump(pgm, d)
},
op::RET => {
    if let Some(re) = self.fstack.pop() {
        self.pc = re;
        Ok(())
    } else {
        Err(RuntimeError::StackUnderflow)
    }
},
~~~

`call` is just a `goto` that pushes the PC to the frame stack. `ret` does nothing but pop the stored PC 
from the frame stack, and turn execution back to that place. 

## A simple call/return program

~~~ title="pgm/call.lva" 
start:
    push_u8 5
    call square
    call square
    out
    fin

square:
    dup
    mul
    ret
~~~

And if we run it: 

~~~
     Running `target/debug/lovas -r pgm/call.lva --print`
Pgm { name: "pgm/call.lva", text: [2, 5, 39, 0, 5, 39, 0, 2, 6, 255, 3, 18, 40], vars: 0 }
Out: 625 (@10)
Terminated.
Runtime=7.643µs
op_cnt=11, pc=10, stack-depth=0, watermark=2
~~~

It outputs `625` &ndash; which is the correct value for `(5^2)^2` or `(5 * 5) * (5 * 5)`.

## What did we build?
We have built something that I would call a subroutine. Basically a `goto` that knows where to return to. 
It does even support nested calls and recursion (if we make sure, that we stop recursion by some `if`). 

What is the call convention? Nothing, to be honest. Just continue, as you where. Global variables are a 
way to communicate values in and out. But we have a stack. We can just push our parameters before we 
call and pop them inside the subroutine. What we push to the stack before we return is the return values, that 
the caller can then pop. That is actually really flexible! We can now write procedures that take as many 
parameters as you want and even return multiple values. 

## Why I don't like it
The simplicity has a charm! But it will be kinda difficult to introduce local variables, without 
braking the parameter simplicity. I feel that the charm works well inside assembler code, but will be less 
elegant in a higher language. There could still be use for this lightweight mechanism.

My major issue is the introduction of an additional stack to the VM. It looks innocent enough, but that 
is because we use a dynamic data structure, a `Vec<usize>`. We use a `Vec` for our main stack as well - but 
actually that is a problem we will need to take care of eventually. I want the VM to run on a system, that 
does not support dynamic memory (yes, I know that I introduced another `Vec`, when I changed `Pgm`). It won't 
be that difficult to remove. But it will come with the cost of having a statically sized stack. 

That is fine 
for limited systems, because you have a limited amount of memory that you can use for the VM, and you want 
to supply it. It then can handle what fits into it and no more. The good thing is, that you know how much that 
is. But if you have more than one stack, you will have to do that for both. And that means balancing your 
limit resources between two different parts; if any of those runs out, you fail. I'd rather have only a single 
chunk of memory to be used. Yes, I could use the same chunk from both ends, as it is custom for systems with 
stack and heap memory. But I don't like to do that here... And there is still the issue of where to put the local 
variables. One more thing we are going to introduce is calling external functions. Our current calling convention 
looks a little weak for that as well.

## Ways out of it
The solution I want to go for is, to put all of my data in a single stack. The frame information should be 
there, in my current opinion. I planed for this for some time; that is why we have that *Frame Base Register*
named as it is.

Why did I write all this then, if I want to get rid of it again anyway? This is a learning journey for me and you. 
We want to build something good towards the end, but the way getting there is part of it!

## Homework
I claimed that you can have nested and even recursive calls with our current means. Write a program that 
proves that claim. 
