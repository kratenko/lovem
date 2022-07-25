---
entry: 32
published: 2022-07-31
---

# Some thought on overhead

__During writing code for the latest entries, there were some thoughts in my head.__

I am in the process of creating a calling convention for functions inside *lovem*. There is a lot of 
forward and back again &ndash; as you would expect when you are trying out different approaches.
Efficiency is, of course in the back of my head during that probing around. My main concern there, as 
mentioned before, is not speed, but size.

And yet I seem to be wasting it without much of a thought. The call operation I used during my previous 
entry pushes 3 values on the stack for every call. That is 192 bytes of overhead! It sounds bad for a 
embedded VM that is supposed to be lightweight. But I simply do not see it as a concern for now.

Optimising the way I push frames to the stack can be done later. I would think, a single `i64` value 
would be enough to hold all of those. How big can the PC become in a constrained VM? A program size 
limit of 64 kiB does notsound far-fetched. And the frame base will not even go up all that way inside an 
embedded system. 64 ki on the frame base register would mean 64 ki values of 64 bytes each, leading to 
a stack size of 4 MiB. That won't happen on my µC. 64 kiB is already a lot. If you want to know how 
much memory you could address with an unsigned 64 bit index on 64 bit values, you will have to 
calculate that yourself.

Introducing that will only make the code more difficult to understand right now. And optimising that 
will be a waste of work on many fronts; I already told you, that I am thinking about changing the 
stack value size radically. So, again: no need there, yet.

Operations are a similar case. I only have a `push_u8`. That is enough for demonstrating stuff. And I 
have a `call`, that needs a `u8` argument to indicate the number of parameters passed. Many functions will 
have only one or even zero parameters. Introducing special `call_0` and `call_1` operations, that do not take 
an oparg, but take that value from themselves will reduce bytecode size a lot later, I think. As well as 
special operations for pushing `0`, `1` and other often used literals. We will do that all at some point.
But only after we have the fundament standing; I do not want to have to update all that code for those 
special operations, when I change fundamental concepts.

As an aside: the opcodes I use for my operations are all temporary. I add them while I work myself through 
this, as I need. I will change them! They are only numbers. Now, that we are using an assembler instead of 
writing bytecode directly, there is no reason not to do that. So don't get too used to that numbers.

I just felt the need, to write this down, just in case some of you were screaming at your screens, because 
of those inconsistencies.
