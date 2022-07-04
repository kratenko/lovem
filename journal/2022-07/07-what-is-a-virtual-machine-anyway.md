---
entry: 7
published: 2022-07-04
---
# What is a Virtual Machine anyway?

So, how do you build a Virtual Machine. There are actually two quite different 
approaches:

* *Register Machine* vs. *Stack Machine*

Let's take a look at those concepts first. This will be very brief and basic.
You can, of course, also have some combination of those concepts, and not 
everything I say here is true for every implementation of virtual machine, but 
it will be close enough for this article.


## Register Machines
Most physical computers are register machines. At least those you will 
be thinking of. You are most likely using one 
right now to read this article. Virtual register machines use the same 
concepts, but not in physical hardware, instead inside another computer as 
software. This allows them to do some things a bit more flexible than a 
real hardware machine would.

A *register* is nothing more than a dedicated place to store a portion of data 
where it can be accessed for direct manipulation. They are more or less a 
variable of the machine's basic data type that have a fixed address, and that 
can be accessed and manipulated directly by the processing unit.
*Register machines* use those to actually compute and change data. All other 
storage places are only that: places where data is put when it is not needed 
at the moment. Register machines have a multitude of registers, from a very few
(maybe 4 or 8 in simplistic designs) to hundreds or more in modern computers.
The size of the registers often gives the architecture its name. E.g. in the 
x86-x64 architecture, that most current CPUs by Intel and AMD are of, a 
register is 64 bits long.

The instructions for a register machine are encoded in *code words*.
A code word is a bunch of bytes that tell the machine what to do in the next 
program step. For simple designs, code words are of a fixed length. This 
code word length is often longer than the register size. So a 16 bit architecture 
could have 32 bit instructions. The reason for this is, that instructions 
consist of an operation code that defines what operation should be executed in 
the next step, but they also contain the arguments passed to that operation.
Because the number and size of arguments needed for an operation differ for 
different operations, decoding the instruction can be quite complicated.
When you put multiple instructions together, you end up with a program. 
This representation of a computer program is called *machine code*. For a 
virtual machine it is also called *bytecode*, although I think this term 
fits better for stack machines (more on that later).

If you want to understand what I tried to describe here, read this really short 
article: [Creating a Virtual Machine/Register VM in C][register-book]. It builds a 
simplistic register VM in C (the whole thing is 87 lines long). It demonstrates 
the principles used in a register machine (fetch, decode, execute), and shows you 
what a *register* is and how it is used. You will understand, how machine code 
is decoded and executed. The article only uses 16 bit code 
words and 16 bit data words (register size). If you know C, you should be able to understand 
what I am talking about in about an hour of reading and coding. If you ever wanted 
to understand how a computer works on the inside, this might be a nice place to 
start, before you read about an actual physical computer. 

A register machine normally has multiple stacks it uses. This does not make it 
a stack machine, those are just needed to store data when it is not currently used.

So a typical operations would be: 
 * "Take the number from register 0, take the 
   number from register 1, add those two numbers together, write the result in 
   register 0."
 * "Take the lower 16 bits of this instruction and write them in register 2."

Lua and Neko are virtual register machines (at least in current versions).

[register-book]: https://en.wikibooks.org/wiki/Creating%5fa%5fVirtual%5fMachine/Register%5fVM%5fin%5fC


## Stack Machines
And then there are *Stack Machines*. They are, I think, easier to understand than
register machines, but following a program during execution is more confusing, since 
the manipulated data is more complicated to follow.

A *stack* is just a pile of data. Data is portioned in fixed sizes, a portion is called 
a word. All you can normally do is put a word on top of the stack - we will call that 
operation a *push*, or you can take the word that is currently on top of the stack 
(if there is one) - we will call that a *pop*. No other direct manipulations of 
the stack are allowed (I say "direct manipulations", because indirectly there often are 
ways that this is done, but that is a detail for later). 

Manipulation of data is done this way by the machine. If you want to add two numbers, 
say 5 and 23, you would write a program that does this: 

  1. Push the first number to the stack.
  2. Push the second number to the stack.
  3. Execute the "ADD" operation.

That operation will pop the two numbers from the stack, add them, and push their 
sum back on the stack (so that after the operation there will be one word less 
on the stack).

A stack machine will also typically have some additional place to store words when you 
do not need them on the stack. These places can relate to variables inside a program.

As you can see from the example above, instructions in a stack machine often do not need 
to have arguments. If data is to be manipulated, it is always on top of the stack. There 
is no need to address its location, as you would do in a register machine. 

Because of this, the instructions for a stack machine are typically encoded in a 
single byte. This byte holds a number we will call *opcode* (short for operation code), 
that simply identifies the operation to execute. If your operation does need additional 
arguments, you write them to the bytes following your opcode byte (the *oparg*), so that 
the operation can read them from your program. This structure of single bytes encoding 
our program is why we call this representation *bytecode*.

The concept of a stack machine is easy to implement in software, but it is not so 
easy to do so in hardware. That is why your typical computer is a register machine.
There are, however, a lot of historical examples of important physical 
stack machines.

The most famous example of a virtual stack machine is the *Java VM*. Java source code is 
compiled to bytecode that is executed inside a virtual machine, the JVM. This vm is 
so common, that many newer programming languages compile to Java bytecode. It makes 
it possible to run programs written in that languages on any system that has a JVM; 
and that includes just about every major and many minor computer systems. A second 
example for a stack machine is the Python VM.


## Some random thought on register and stack machines
While writing this down, describing the two kinds of machines I couldn't help but 
notice a curious fact:

A register machine manipulates data inside addressable registers. When 
the data is not need, it can be stored away in some kind of stack.

A stack machine manipulates data inside a stack. When the data is not needed, 
it can be stored away in some kind of addressable spaces, not unlike registers.

It looks as if you just need both concepts to work efficiently.
