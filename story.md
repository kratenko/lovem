# Lovem Again!
*2022-06-24*

So, I am back at writing my Low Overhead Virtual Embedded Machine. From scratch. 
Everything I had was dumped (have the code still, somewhere, but it is okay to start 
anew - I learned during my previous attempts). 

## Why?
Why am I doing this? Well, that has a 
history. Basically, I am writing firmware at work for our IIoT devices. They are 
pretty versatile, so configuring them tends to be rather complicated. And still, I 
want them to be able to do more: react to situations depending on sensor data, 
prepare data read from sensors, so that it transmitted with less overhead, etc.
Right now, that would mean writing custom firmware for those customer cases
(in C) and deploy it for their devices - and maintain those firmwares over years. 
And no one wants to pay for that! Nor do I care to do the maintaining.

What's the alternative? Add those features to the standard firmware and add more 
configuration features. Great. So it will be even more complicated. And every second time 
you have a new use case, you will find your current solution insufficient, so you 
need to modify your firmware again to include that one more special case. And make 
your config more powerful (please keep it backwards compatible, while you at it, thank 
you very much - remember, there are thousands of devices out there, that still need 
to work with their configuration, when the firmware update hits them).

And your config? You want to be able to react to triggers, and you want to do react 
in any random way. And you want to be able to manipulate your data points in any 
way needed. So when you walk that road for some time, you will end up with a configuration 
that is basically a programming language, since that is the only thing powerful 
enough, to do all that. And it will be a badly grown one, you can be sure about that!

So let's embrace that consequence, and simply start with using a scripting language 
as means for advanced configuration! We will end there, cut some corners on the journey!

## Sorry, what are we trying to do again?
We are in need of a scripting language that runs on a very constrained device. Think of 
a microcontroller that has 352 kiB flash space and 191 kiB RAM for our complete firmware. 
And keep in mind that most of the behaviour of our device will not be implemented in 
the scripting language. There will be a number of hooks that should give control to 
the user supplied script, which will execute for a very short time, collect some data 
from sensors, act on them (maybe control actuators, but mostly generate data to 
be uploaded), and then return control to the firmware. And yeah, we will need to store 
the "script" somewhere on that device, so it would be great if it was not multiple 
kiB of program. I could use an SD-card in the dive (so I guess could store 1 TiB of 
script on the device if I needed), but those are not that reliable and are an optional 
extension that could already have a different use.

## So you had this great idea about that wheel...
There are many wheels out there, why o why do you want to invent it again? Well, are 
there, though? Because that is what I thought. I started looking at what I know. Soo...

### Lua
[Lua][lua] is a tested scripting language to use in host languages like C. I first 
experimented with it when I was trying to write games for fun, back in the early 2000s.
I was somewhat intrigued when I came upon it again some 10 years later while playing 
heavily modded Minecraft. In [ComputerCraft][computercraft] you have a block that is a 
computer, which you can write programs for in Lua. It even has a little operating 
system where you store your files (edit them, horribly, in an editor on an 
in-game monitor), execute programs that you can store on in-game floppies to carry 
around. It was a horrible kind of fun to do just anything inside that world.

Lua was invented to solve a similar sounding problem: scripting in computer games. 
Level designers, story writers, etc. should not be bothered with having to write 
C-code to achieve their tasks (and re-compiling during developing those is *not* the 
way). So yeah, that is, more or less, my problem. And you can even compile Lua to 
byte code which is run in the interpreter. Neado!

But, oh, the interpreter... turn's out, it is quite big! At least when you are 
working with embedded hardware. To quote [lua users][lua-users]:

> Smaller footprint than Python. e.g. Look at the size of python22.dll, 824kb. A basic 
> Lua engine, including parser/compiler/interpreter, but excluding standard libraries, 
> weighs in at under 100kb.

That's fine and all, but still a bit much for me - to be fair, I would need neither 
parser nor compiler. Other sources give numbers like <300 kB - which is overkill.
I did compile it for our architecture - and the VM alone, without any of our own 
code doing stuff, exceeded the flash size I had. This 
[stackoverflow question][stackoverflow-elua] quotes the eLua FAQ to recommend 
256 kB flash and 64k kB RAM which is too much for me - at time of writing this, 
eLua documentation seems offline in parts, so that does not give me confidence either.
Quote from an [answer to that question][stackoverflow-elua-answer]:

> I would recommend LUA (or eLUA http://www.eluaproject.net/ ). 
> I've "ported" LUA to a Cortex-M3 a while back. From the top of my head it had a 
> flash size of 60~100KB and needed about 20KB RAM to run. I did strip down to the 
> bare essentials, but depending on your application, that might be enough. 
> There's still room for optimization, especially about RAM requirements, 
> but I doubt you can run it comfortable in 8KB.

Back then I found a post I cannot find again that claimed, you can get the footprint of 
the Java VM smaller than that of the Lua VM (if you cut standard lib, which is part 
of Java and not of its VM). That sounds possible to me, when you have a glimpse on how 
those languages work. But then again you would not have any of the parts you are used to in 
Java. Also, there are some thoughts on how fitting that language is for my case, I'll 
have something about that later on.

So I dropped that. 

[lua]: https://www.lua.org/
[computercraft]: https://www.computercraft.info/
[lua-users]: http://lua-users.org/wiki/LuaVersusPython
[stackoverflow-elua]: https://stackoverflow.com/q/1082751/1358283
[stackoverflow-elua-answer]: https://stackoverflow.com/a/1087182/1358283

### Java VM

So... to the JVM then? To be honest: I do not want to go there. It does not feel right!
JVM does not mean Java, I know that. I could use the VM and create my own language that 
compiles to this highly optimised VM. I could use any of those many languages that 
already compile to Java bytecode. And yes, JVM does not equal Oracle; there are free 
open JVM implementations out there. I admit I did not try to find out how small that 
VM would be. But it just feels so wrong on so many levels. I simply cannot imagine 
JVM is the tool for the task. As I teasered for Lua before, more thoughts on this later.

But: no.

### JavaScript

Where do I begin? How about here: *no*

I did not even try to find a solution for running JavaScript on the device. I am sure 
there are some. But so there are reasons against using this language. Once again, more on 
that later, when I reflect more on my use-case.

### Python

I do like Python. But it is pretty big. There are some broken projects like tinypy. That 
looks dead. And there is, of course [MicroPython][micropython].

> MicroPython is packed full of advanced features such as an interactive prompt, 
> arbitrary precision integers, closures, list comprehension, generators, 
> exception handling and more. Yet it is compact enough to fit and run 
> within just 256k of code space and 16k of RAM.

That 256k is a pretty big "just" for my liking. It is meant for the pyboard, having 
an STM with 1024 KiB flash ROM and 192 KiB RAM. And that device will not have 
a main firmware "next to it". So again, not really my use-case.

[micropython]: https://micropython.org/

### Are there others?
I googled. I looked at quite a few of them. It never feels close to what I want. 
First of all I found, that "embedded scripting" is a term that most of the time is 
not meant as in "embedded device". That's because the scripting language itself is 
what is embedded in the host language (be it C, Java, Rust, or whatever). Lua is a 
prime example on that terminology problem. So what I am really looking for 
is an "embedded embedded scripting language". Good luck on googling that!

There are projects that try to be what I am looking for. Few such projects seem to 
be in a state that I would by willing to use them in a commercial product. Think 
long term maintainability here.

And, again, they often do not aim at my problem very well. They want some ease of 
usage, which is fine, but they tend to have a too-high-level approach for my linking.
Yes, I will start to talk about what I mean, soon.

Maybe I should have taken a closer look at languages like [Neko][neko]. But the 
first impression was hinting at many of the problems I try to describe here.

No language was sticking out. I did not spend much time on any other language.

[neko]: https://nekovm.org/


### Conclusion
So, languages are never a good fit on what I want. They are hard to integrate 
in my existing system. They are too big. They are often not well maintained.

Is this already the end of my journey? It does not have to be. But it will be 
a very different journey, if I proceed.


## That use-case I was talking about
I was mentioning it, was I not? Languages do not seem to fit very well on by problem.
What do I mean by that?

I am doing very low level stuff. I am pushing bytes, often even bits around. Imagine 
receiving a bunch of raw bytes from a sensor attached via UART. You dump them in 
a buffer. The task for the script is now, to parse a few specific bytes out of that 
buffer, and make sense of them. Some are uint16 integers in little endian. Others are 
int32, spread over two uint16 BE registers, that are not next to each other, and you 
need to combine the two uint16 BE values in LE order to get your value. This scenario 
is fictional, but much more likely, than you would expect.

All this sound horrible, and it is sometimes tricky, but of course you can do all this 
in any language that gives you access to bytes in any way. If you ever worked with 
LoRaWAN, you might have had to do such things in your network server (e.g. [TTN][ttn]), 
to parse your uploaded data from bytes into, say, JSON. On many network servers you 
can do so with your own scripts (hey, that's close to what I want to do). And they give 
you the language suited best for this kind of problems: JavaScript.

No, really. You are doing bit-manipulation on your bytes in a language where every 
number is stored as a float. You push your data around in JSON, a format that does 
not support byte arrays, so you have to communicate your bytes encoded in base64 or 
hex and store those inside strings. And you hope that the receiving end is able to 
decide if the date should be interpreted as a string or as hex or as base64 (and for 
hex strings, all of that can be possible at the same time).

That is a problem, that I have with most scripting languages that I encountered. 
You get a giant infrastructure supporting classes with multiple inheritance support 
and polymorphism. You get on-the-go code interpreting. You get asynchronous execution 
support, dynamical typing, garbage collection, and whatnot. 

And I want to write a function, that is called when needed, and gets handed a few
bytes. I want it to extract a few of those bytes, interpret them as a number, 
compare that number to a threshold, and if the value exceeds said threshold, 
call a different function with a few bytes, that are then send back over some 
peripheral (but that is not for the script language to control, just pass them 
to the system).

Those languages tend to have a huge set of features that I do not need (or even to not 
want to have), while lacking many features that would be useful to me. So all that 
features would have to be implemented by me somehow, anyway.

You see now, why I cannot find any language that I like?

[ttn]: https://www.thethingsnetwork.org/

## Script it then!
Okay, okay. Let's say you bought my argumentation. Go ahead, hack together some 
scripting, knock yourself out. Just parse it in your firmware and execute it.

Yeah, I could do that. Simple syntax. Parse it on the fly. Store variables in 
some hashmap, execute functions by name, have them call functions supplied by the 
firmware to interact with the hardware. And you can just throw those scripts 
into your git repo. Easy peasy. Only it wouldn't. But that language would 
grow oh ever so horribly. And it would never be good. Ever tried to parse an 
expression like `f = 3 * a + (b + 2) * 1.2`. In C? And that expression is not too 
complex even. There would be so many parsing errors that only happen at runtime
(on the device, remote, in the field, without any logging, let alone debugging).
Besides: I do not want the complicated (read: big) parsing code on all of my devices. 
That is space (and execution time, which translates to power usage) that could be 
done once, on a more powerful device that I can monitor directly (that is: my laptop).
Also: source code is long! I will need to store that on my device somewhere.
And trying to write source code extra short makes it even worse.

## Let's get virtual
So what is the solution here? We need a virtual machine that executes programs 
precompiled into bytecode. And we want that VM to be lightweight. If you design 
it carefully, a VM can be pretty small. What bloats things up often is the 
standard library with all the tools you need to efficiently write programs.
But I do have a mighty host language (C, mostly), that already has a huge 
library of functions ready to be used (and which are often used already and 
henceforth already inside my firmware). I only need to provide a wrapper, that 
exposes them to my VM, and I can have them all: sinus/cosinus, logarithms, 
AES-encryption, Ethernet. You name it, we got it (well, most of it... be sensible.. 
we should at least be able to find an implementation somewhere...).

And the best part? I postpone the pain of having to design the language. If you 
have a solid VM that supports the operations you need to get your work done nicely, 
you can pretty much design a language any way you want. You just need a bytecode 
compiler. You can even have multiple languages, in case you have too much time on 
your hands. But more important: you can develop the language without needing to 
change your VM (if you know what you do and if you plan well enough). That means: 
no need to update the firmware on your devices everytime the language advances.
As long as your bytecode stays compatible.

Is it realistic to finish this project, maybe even, to build something good?

I highly doubt it. This is a huge project, if I make it all I want it to be. 
But at least I have learned quite a lot on the way so far. Why do you think 
I threw everything away (for the second time) and started on an empty board?

So I guess, time is not wasted?



# What is a Virtual Machine anyway?
So, how do you build a Virtual Machine. There are actually quite different 
approaches. The most important difference (at least to me) is:

* *Register Machine* vs. *Stack Machine*

Let's take a look at those concepts first. This will be very brief and basic.
You can, of course, also have some combination of those concepts, and not 
everything I say here is true for every implementation of virtual machine, but 
it will be close enough for this article.


## Register Machines
Most physical computers are register machines. You are most likely using one 
right now to read this article. Virtual register machines use the same 
concepts, but not in physical hardware, but inside another computer as 
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
virtual machine it is also *called bytecode*, although I think this term 
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

Lua and Neko are register machines (at least in current versions).

[register-book]: https://en.wikibooks.org/wiki/Creating%5fa%5fVirtual%5fMachine/Register%5fVM%5fin%5fC


## Stack Machines
And then there are *Stack Machines*. They are, I think, easier to understand than
register machines, but following a program during execution is more confusing, since 
the manipulated data is more complicated to follow.

A stack is just a pile of data. Data is portioned in fixed sizes, a portion is called 
a word. All you can normally do is put a word on top of the stack - we will call that 
operation a *push*, or you can take the word that is currently on top of the stack 
(if there is one) - we will call that a *pop*. No other direct manipulations of 
the stack are allowed (I say "direct manipulations", because indirectly there often are 
ways that is done, but that is a detail for later). 

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
arguments, you write them to the bytes following your opcode byte, so that the operation 
can read them from your program. This structure of single bytes encoding our program is 
why we call this representation *bytecode*.

The concept of a stack machine is easy to implement in software, but it is not so 
easy to do so in hardware. That is why your typical computer is a register machine.

The most famous example of a stack machine is the *Java VM*. Java source code is 
compiled to bytecode that is executed inside a virtual machine, the JVM. This vm is 
so common, that many newer programming languages compile to Java bytecode. It makes 
it possible to run programs written in that languages on any system that has a JVM; 
and that includes just about every major and man minor computer systems. A second 
example for a stack machine is the Python VM.


## Some random thought on register and stack machines
While writing this down, describing the two kinds of machines I couldn't help but 
notice a curious fact:

A register machine manipulates data inside addressable registers. When 
the data is not need, it can be stored away in some kind of stack.

A stack machine manipulates data inside a stack. When the data is not needed, 
it can be stored away in some kind of addressable spaces, not unlike registers.

It looks as if you just need both concepts to work efficiently.



# Making virtual a reality
So I have been talking a lot about VMs without doing anything concrete. Well 
that is not true, I have done quite a bit already, but I am still describing 
earlier steps. We will get there.

## Registers?
When I was looking around a scripting language to use inside our embedded 
devices, I came across an article I mentioned in an earlier post: 
[Creating a Virtual Machine/Register VM in C][register-book].

Reading it made me want to try working with a register machine, mainly because I 
have not been stuff like this since my early semesters. Never hurts to refresh 
rusty knowledge.

So I started designing a register VM, starting from that code, but more complex, 
with longer data words and longer instruction words, more registers, and so forth. 
For this project I came up with *lovem* as a working title. It still stuck to now, 
two approaches and a year later. I also started implementing some concepts I still 
want to add to lovem in my current approach, but that is for a later post to 
discuss.

I was experimenting with a quite complicated instruction word encoding. I was trying 
to fit everything in a few bits (32 of them if I recall correctly) with varying 
instruction code length and quite long arguments. I wanted to include instructions 
on three registers, which takes up quite some bits to address. Of course, you can 
get away with two register operations only - or if you are fancy you can even use 
a single address or even no address for most instructions. You will just end up 
with a lot of register swapping. I guess my rational for having three addresses in 
an instruction was code size. For what I want to do, 32 bit instruction words feel 
quite long (4 bytes per instruction!). And every swap would mean another 4 bytes of 
program size. So trying to optimise for fewer operations by having more flexible 
instructions. 

I do not even know if that rational makes sense. I guess I would have needed to try 
different layouts to find out. Or maybe read more about that topic, other people 
have done similar things I assume. But I never got that far. The experiment showed 
me, that I do not want to build lovem as a register machine. I think building a 
clever register based architecture for my goals would make it too complicated. 
I want simple. To reduce the VM's overhead, but also on principle. Complexity is 
the enemy.

I'm pretty sure, that code still exists somewhere, but there is no sense in 
publishing it or even in me reading it again, so you will never see it. I think 
of it as a pre-study with a very useful conclusion: *not* a register machine.

[register-book]: https://en.wikibooks.org/wiki/Creating%5fa%5fVirtual%5fMachine/Register%5fVM%5fin%5fC


## Stacks!
So a stack machine it is! I have looked at a few during my research for lovem,
looking at instruction sets and design ideas. It is not the first time, I have been 
working with those. In a different project (around the same time I started work on the 
register based machine), I was starting to implement a stack machine. That one had 
a different aim and therefore very different challenges. It was more of an object-oriented
approach with dynamic program loading and calling code in different programs. It could 
do quite a few things already, but it will never be continued. I learned a bit 
about calling conventions and found out that it is not so simple, when you want to 
switch between multiple programs and objects. That is where the project got too 
frustrating for me (and some external events made it obsolete, so that is okay). 
But I take it for a pre-study on stack machines and calling conventions. Not that 
I have developed a proven concept for it, but I know about the problems there...

I had a PoC for lovem as a stack machine back then, too (right after I ditched the 
register approach). That code won't be published either, but the attempt showed me, 
that I want to take that road for a serious approach on creating lovem.


## Onwards
I guess this concludes the prehistory of the lovem story. I am, for whatever reason, 
back on the project, currently with a decent amount of motivation. You never know 
how long that lasts, but right now I like the idea of continuing the development
(I don't on keeping the current pace), while talking about the development process, 
sharing my thoughts on decisions I make. Next post should start on sharing newer 
thoughts.



# Start again!



# Gotta love 'em Chunks
*2022-06-24*

So I think it would be a nice idea to give lovem native support for chunks. 
You cannot, of course, have any idea what I mean by that.