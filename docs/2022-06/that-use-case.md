---
title: That use-case I was talking about
author: kratenko
author_gh_user: kratenko
entry: 3
publish_date: 2022-06-27
starter: "This is why every existing scripting language is objectively bad! Sorry, I wanted to say: This is my problem and languages do not seem to be designed for it."
---

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
