---
entry: 10
published: 2022-07-10
---

# It looks so weird

__Now, that you have seen some code, I might have to explain a bit again. Depends, on where you 
are coming from, I guess.__

So, did you take a look at the code, yet? In case you've forgotten, this is my "initial commit":

https://github.com/kratenko/lovem/tree/v0.0.1-journey

It is not the original initial commit, as I did commit way too late, and it was not suitable for 
writing a story about it. So I created a new, clean version, with just very simple concepts that 
I can explain in a single entry. In the next entry, that is.

If you are thinking: "What is that weird source code?", then you are in for a real treat (and a lot of pain), 
should you chose to follow up. The code you are seeing is written in [Rust][rust].

## Once again: but why?
Why Rust? Because Rust! Writing Rust can feel so good! And for something like a VM, it is such a good choice. 
If you have never heard of the language (or heard of it, but never looked into it), it is hard to understand 
why this is so. My advice: try it! use it! Or read along this journal, code along, you might like it.

When you start, chances are high that you will *not* like Rust. 
The compiler is a pedantic pain in the ass. But at the same time it is incredibly polite, trying to 
help you find out, what you did wrong, and suggesting what you might want to do instead.
And Rust really, really tries, to keep you from shooting yourself in the foot. It 
tries to make common mistakes impossible or at least hard to do &ndash; those mistakes that happen everywhere 
in C/C++ programs and their like. Yes, those mistakes that are the cause of the majority of all security 
problems and crashes. Buffer overruns, use after free, double free, memory leak &ndash; to name just some 
common ones from the top of my head. And Rust makes all it can to make those mistakes impossible *during 
compilation!* So it does not even add runtime overhead. That is so powerful!

And it is so painful. Half of the things you do, when writing C/C++, you will not be able to do in Rust 
in the same way. Every piece of memory is owned. You can borrow it and return it, but it cannot be owned 
in two places at once. And if any part of the program has writing access to it, no other part may have 
any access. This makes some data structures complicated or impossible (there are ways around it), and you 
will have to think quite differently. But if you give in on that way of thinking, you can gain so much. 
Even peace of the mind, as the coding world will look a lot saner inside Rust source code. This will, of 
course, come with the price, that all code in other languages will start to feel dirty to you, but that 
is the way.

Also, there are a lot of ways to write code, that you cannot add to a language that already exists. 
C and C++ will never be freed of their heritage; they will stay what they are, with all their pros 
and cons. Things are solved differently in Rust. Did I mention there is no `NULL`? And I have never missed 
it for a moment. Rust solves the problems other languages solve with `NULL` by using enums. That comes 
with certainty and safety all the way. There are no exceptions either. That problem is also solved 
by using enums. The way the language embraces those, they are a really powerful feature! And there are 
lot more convenient ways of organising code, that I keep missing in my daily C/C++ life.

I will not write an introduction into Rust here. At least not your typical "how to get started in rust"
intro. There are a lot of those out there, and I am already 10 posts into my Journal without programming.
Maybe the Journal will become a different kind of Rust introduction, as it will try to take you along 
a real project, as it develops, from the beginning on. I will run into problems along the way and try to 
solve them in Rusty ways. This might be a good way, to start thinking in Rust. But, to be honest, I 
did never finish a project in Rust, yet. I got quite a bit running and functional, and I think in 
some parts in a rust-like way. But this is for me as much as anyone else as a learning project. I will 
make weird things. But the basics, I have worked with, yeah.

The initial learning curve will be steep! I try to not get too fancy in the first draft, so the code 
will not be good Rust there! So, if you are shocked at how bad my Rust is &ndash; it will be very 
different, soon. But I want to give everyone a fair chance to hop on without understanding all the 
concepts. The initial code should be not too hard to follow, if you know C/C++, I hope. Learning a new 
thing (writing a VM) in a new, quite different language is a mouth full, I know.


## Didn't you say, you use C/C++?
Yes I did say that. And I do use those. It is not easy to change that, when you have a certain amount 
of legacy code (and not much experience with the new language, as we do not really have, yet). But we 
do have a saying these days. Often, after a debugging session that lasted for hours, when we find the bug, 
understand it and fix it, there is this realisation, that fits in the sentence:

"Mit Rust wär' das nicht passiert." &mdash; "This would not have happened with Rust."

So, this will not happen to me with this project, because those things will not happen with Rust!

[rust]: https://rust-lang.org
