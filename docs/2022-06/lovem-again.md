# Lovem Again!

__It seems like that dude dug out one of his time-wasting projects and put some 
more work into it. This time in a public repo even. And for some reason he wants 
to let the world know, how and what he is doing. The journey starts here.__

:octicons-book-24: Entry \#1 ·
:octicons-calendar-24: 2022-06-24
---

So, I am back at writing my *Low Overhead Virtual Embedded Machine*. From scratch. 
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
