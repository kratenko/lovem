---
entry: 4
published: 2022-06-28
---

# Script or virtual

__After the quest for a scripting languages failed, we plan writing our own.__

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

So I guess, my time was not wasted, huh?
