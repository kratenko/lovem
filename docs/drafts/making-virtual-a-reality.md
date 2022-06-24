# Making virtual a reality
:octicons-book-24: Entry \#3 ·
:octicons-calendar-24: 2022-06-24 ·
:octicons-clock-24: 3 min read
---
So I have been talking a lot about VMs without doing anything concrete. Well 
that is not true, I have done quite a bit already, but I am still describing 
earlier steps. We will get there.

## Registers?
When I was looking around a scripting language to use inside our embedded 
devices, I came across an article I mentioned in an earlier post: 
[Creating a Virtual Machine/Register VM in C][register-book].

Reading it made me want to try working with a register machine, mainly because I 
have not been stuff like this since my early semesters. Never hurts to refresh git@github.com:kratenko/lovem.git
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
