---
entry: 16
published: 2022-07-17
tag: v0.0.3-journey
---

# Early VM decisions
__Many design decisions must be made for lovem. Here I talk about some of those in the current state.__

I have shared and discussed source code in the recent posts. Now it is time again, to write about 
design decisions. I made a few of them for the code you saw. So far I have not been reasoning about 
those here, and some of you might have wondered already. Let's talk about them.

Let me remind you: *lovem* is a research project for myself. And an education project for myself as well. 
None of my choices at this stage are set into stone. I *will* make lots of mistakes that I will be 
changing later. I even choose some paths, that I know I will be leaving again. I might just take any solution 
for a problem, at this stage, as I do not know, what is the right choice. So start somewhere, see where it goes. 
Some of those are deliberately weird or bad choices, but they make things clearer or simpler at this stage. 

Let us address two of those choices you can find in the current source code.

## Word size
I talked about register sizes defining architecture, back in 
[What is a Virtual Machine anyway?](../2022-07/what-is-a-virtual-machine-anyway.md). And then I went 
totally silent about that topic and just used `i64` as type for my stack. Is that a good idea? 
I used it for simplicity. The idea goes back to when I was experimenting with using a register machine 
for lovem. Having a simple datatype that can handle big values seems simple. After all, other languages/VMs 
use some version of *float* as their single numeric datatype:

**JavaScript**
> **JavaScript Numbers are Always 64-bit Floating Point**
> 
> Unlike many other programming languages, JavaScript does not define different types of numbers, like integers, short, long, floating-point etc.
> 
> JavaScript numbers are always stored as double precision floating point numbers, following the international IEEE 754 standard.
>
> &mdash; <cite>[w3schools.com][js-floats] - retrieved 2022-07-11 </cite>

**Lua**
> **2.3 - Numbers**
>
> The number type represents real (double-precision floating-point) numbers. Lua has no integer type, as it does not need it.
>
> &mdash; <cite>[Programming in Lua][lua-floats] - retrieved 2022-07-11

Well, reducing complexity is good. But having each little number you use in your programs eat up 8 bytes of 
memory does not sound low overhead to me. And that is, after all, the goal. So I guess, that will change in 
the future. But let's keep it for the time being. There will be some interesting things we will be doing in the 
near future; even if we might dump those features later. I already implemented them during the early phase
(when I was not writing a public journal), so not adding them here would be insincere. Having 64 bit values 
is a part of our journey.


## Opargs
I have no glossary, yet, so you have to live with me inventing terms on the spot. I used that word in the 
source code already. What I mean by it, are the arguments to an instruction inside the bytecode, that follow 
the opcode and influence the operation. They are the arguments you give inside your program's code.

As of `v0.0.3-journey` we only have a single opcode that takes an oparg, and that is `push_u8`. You can see 
how there is a `fetch_u8()` instruction in the code that handles that operation, and none in the other 
operations. See [`execute_op`][execute_op]. 

So we have different behaviour depending on the opcode. `push_u8` fetches an additional byte from the 
bytecode, the other opcodes do not. Existing VMs handle this differently. The Java VM, for example, 
has a dynamic number of opargs, too. They call them *operands*:

> *2.11. Instruction Set Summary*
> 
> A Java Virtual Machine instruction consists of a one-byte opcode specifying the operation to be performed, 
> followed by zero or more operands supplying arguments or data that are used by the operation. 
> Many instructions have no operands and consist only of an opcode. 
> 
> &mdash; <cite>[The Java® Virtual Machine Specification - Java SE 8 Edition][jvm_opargs] - retrieved&nbsp;2022-07-11</cite>

The Python VM on the other hand, uses exactly one byte as oparg on all instructions 

> The bytecode can be thought of as a series of instructions or a low-level program for the Python interpreter. 
> After version 3.6, Python uses 2 bytes for each instruction. 
> One byte is for the code of that instruction which is called an *opcode*, 
> and one byte is reserved for its argument which is called the *oparg*.
>
> [...]
> 
> Some instructions do not need an argument, so they ignore the byte after the opcode. 
> The opcodes which have a value below a certain number ignore their argument. 
> This value is stored in `dis.HAVE_ARGUMENT` and is currently equal to 90. 
> So the opcodes >=`dis.HAVE_ARGUMENT` have an argument, and the opcodes < `dis.HAVE_ARGUMENT` ignore it.
>
> &mdash; <cite>[Reza Bagheri - Understanding Python Bytecode - in Towards Data Science][pvm_opargs] - retrieved&nbsp;2022-07-11</cite>

That does remove some complexity. And adds new complexity - for opcodes with more than one oparg byte - they 
exist in python and are handled with a special opcode, that adds an additional oparg byte. I think it will make 
execution faster, as fetching can be done it advance. If you do not know, how many bytes you need, before 
your read your opcode, you cannot prefetch the next instructions.

For our goal, keeping the bytecode small is much more important than execution time. So I am pretty sure we 
will stick with the dynamic number of oparg bytes in lovem.

[js-floats]: https://www.w3schools.com/js/js_numbers.asp
[lua-floats]: https://www.lua.org/pil/2.3.html
[execute_op]: https://github.com/kratenko/lovem/blob/42b373bccf7e761626d424fb9ba8252108805d9d/src/vm.rs#L95
[jvm_opargs]: https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11
[pvm_opargs]: https://towardsdatascience.com/understanding-python-bytecode-e7edaae8734d