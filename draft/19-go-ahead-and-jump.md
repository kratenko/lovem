---
entry: 19
published: 2022-07-31
tag: v0.0.6-journey
---

# Go ahead and jump!

__All our programs have been linear so far. Let's build the base for jumping around.__

In every program we have written so far, each instruction just advances the PC[^1], until we reach the end.
That is very linear. 

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

[^1]: PC: the Program Counter, a special register that points to the next instruction to be executed.
