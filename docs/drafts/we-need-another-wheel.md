# We need another wheel

__There are many wheels out there, why o why do you want to invent it again? Well, are 
there, though? Because that is what I thought. I started looking at what I know. Soo...__

:octicons-book-24: Entry \#2 ·
:octicons-calendar-24: 2022-06-24
---

## Lua
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

## Java VM

So... to the JVM then? To be honest: I do not want to go there. It does not feel right!
JVM does not mean Java, I know that. I could use the VM and create my own language that 
compiles to this highly optimised VM. I could use any of those many languages that 
already compile to Java bytecode. And yes, JVM does not equal Oracle; there are free 
open JVM implementations out there. I admit I did not try to find out how small that 
VM would be. But it just feels so wrong on so many levels. I simply cannot imagine 
JVM is the tool for the task. As I teasered for Lua before, more thoughts on this later.

But: no.

## JavaScript

Where do I begin? How about here: *no*

I did not even try to find a solution for running JavaScript on the device. I am sure 
there are some. But so there are reasons against using this language. Once again, more on 
that later, when I reflect more on my use-case.

## Python

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

## Are there others?
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


## Conclusion
So, languages are never a good fit on what I want. They are hard to integrate 
in my existing system. They are too big. They are often not well maintained.

Is this already the end of my journey? It does not have to be. But it will be 
a very different journey, if I proceed.
