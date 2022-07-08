# Turn fragile into rusty
Did you play around with the program from the previous post? If you are new to Rust, you really 
should. At least mess around with our bytecode. You should find, that our VM does not react well to 
errors, yet. It simply panics! That is no behaviour for a respectable rust program.

We will make it more rusty, look at the enhanced version:

Repo:
https://github.com/kratenko/lovem/tree/9d97281bd6ffdae894f8052c91ea32d1d761fdb2

main.rs:
https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs

If you do not know your way around Rust, some of those things will be difficult to understand. It might be 
time to read up on some Rust, if you intend to follow my journey onwards. I will not explain everything here, 
but I will give you some leads right now, if you want to understand the things I did in that change.

The most important thing to understand for you will be Enums. 
Yeah, I know. That is what I thought at first learning Rust. 
"I know enums. Yeah, they are handy and useful, but what could be so interesting about them?"

Well, in fact, enums in Rust completely change the way you are writing code. They are such an important 
part of the language that they have an impact on just about every part of the language.

I introduced an enum to the code:

~~~rust
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    InvalidOperation(u8),
    StackUnderflow,
    StackOverflow,
}
~~~

It is obviously a datatype to communicate runtime errors of different nature. And I use it a bit 
like you would exceptions in some other languages.
Nevermind the `#[derive...]` part for now. That is just for fancy debug output (and a bit more).
Once you understand [line 33][line33]: `InvalidOperation(u8),`, you are on the right track!

If you know what happens in the return type of fn `push` in  [line 70][line70], you are golden.
The `Result` type can communicate a value on success or an error condition on failure. The great 
difference to typical exceptions form other languages is, that there is no special way to pass on 
the errors, as with exceptions that are thrown. It is just your normal `return` statement used.
And this is done, you guessed it, with enums. If you want to read up on `Result`, try 
understanding `Option` first. I am using that in my code, even though you cannot see it.

If you are wondering now about the return of fn `push`, that does not have a `return` statement 
to be seen, you should find out, while some of my lines do not have a semicolon `;` at the end, 
while most do. 

And then there is that tiny `?` in [line 101][line101].

Also find out what happens in the `match` in [line 166][line166]. It might help if you start 
with the `if let` statement.

Bonus points: [line 65][line65]. If that is clear to you, you need have no worries, you 
are into enums and how to use them

## Homework
So, this is what will get you through a lot here, try that order:

  1. `Option`
  2. `Some(v)` vs. `None`
  3. `Result<v, e>`
  4. `Ok(v)` vs. `Err(e)`
  5. `if let Some(v) = `
  6. `match`
  7. `Result<(), e>`
  8. `Ok(())`
  9. `unwrap()`
  10. `?`
  11. Bonus: `ok()`, `ok_or()`, and their likes

If you understand for each of those, why I put them in the list, you are prepared to handle most Rust 
things I will be doing in the next time. If you have problems with parts of it, still, move on. It 
gets better when you use them.

[line33]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L33
[line65]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L65
[line70]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L70
[line101]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L101
[line161]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L161



# To the library






# [...] ----------------- far in the future ---


# We have variables
"A stack alone is not that mighty. But now we can store data away."

I implemented variables for the VM. And I did it in a way, that will freak out 
programmers, who have only ever worked with high languages in well behaved 
environments &ndash; we know have variable support, but for global variables only.

Why would I do that? Well, it was easy. You might be surprised how easy it was. 
And it helps a lot in having something useful. For what I am going for, it would 
actually be a viable thing to. You could do a lot. But don't worry, I want local 
variables, too.

## Tell me were to stick it
Variables need to live somewhere. When I first talked about stack machines, I 
said that "no other direct manipulations of the stack [were] allowed [but push or pop]." 
We will now see, why I said that.

Variables hold values; words, to be more precise. We have an entity, that can hold an 
arbitrary number of words: the stack. So, what is the idea? When I write a program, 
I will know how many variables it will need. Actually, my assembler now can do that for me.
When I pass the program to the VM for execution, it looks at that number, and pushes that 
many zeros on the stack. Then it marks the current stack position as the new bottom. 
It does that by the newly introduces special *Frame Base Register (FB)*.

What's with that funny name? This is something I will need later, when I introduce real 
function calls inside the VM. A call will create a new frame that is somewhat like a 
new local execution environment. This will also allow for local variables (told ya, I want those).
But for know I have up to 256 global variables at my disposal. 

There are two new operations for that [^1]:

  * `store_g`: pop value from the stack and store it in the global variable identified by the 1-byte oparg.
  * `load_g`: read value from global variable identified by the 1-byte oparg and push it to the stack.

So, the vm now can read and write values on the bottom of the stack. Does that mean, that we do not have 
a stack machine? Well no, it's okay. We can neither insert nor remove values anywhere bot at the top. 
A machine that can really only see the top of a single stack would be called a [pushdown automaton][pushdown]. 
The name is inspired by tray or plate dispensers, as you would find in a cafeteria; those things where the 
stack is pushed down by its weight, so that you only ever see the top of it. I am absolutely fine with not 
building a pushdown automaton, as those are not Turing complete - and Turing completeness is, what we 
definitely want lovem to be!

At this point, lovem is actually really quite able. It should be Turing complete with the global variables
(hmm - I would need to add some more opcodes to be really useful; some logic would be nice). But worry not, 
we are far from being done. There is still so much to do.

[^1]: At this point, all opnames can be subject to change, as I do not have a consistent naming scheme yet.
[pushdown]: https://en.wikipedia.org/wiki/Pushdown_automaton


# Gotta love 'em Chunks
*2022-06-24*

So I think it would be a nice idea to give lovem native support for chunks. 
You cannot, of course, have any idea what I mean by that.

...


# Alles ist aus Stack gemacht
As I wrote earlier, I have this plan to implement some handling of byte buffers at the 
very basic level of lovem. Direct support in bytecode, is what I am thinking of.
The question that I need to solve for that: where do I put those?

I am a virtual machine in a mighty host language. I could just malloc them on demand. But 
then I will have some sort of pointers to them. I would like to avoid pointers very much. 
I do not know if that is feasible. But more than that: would I need a ref counter then, 
a garbage collection. No, no, no! That is not good! Besides, "just malloc" them is a 
pretty big just. I am aiming at limited devices. They might not have dynamic memory. 
Allocation has a huge impact on complexity. And it becomes really hard to know if your 
system with malloc will ever run out of memory after a long run with uncertain circumstances.
That is why embedded projects sometimes have the demand, that no dynamic memory must be 
used. Malloc is not an option.

## Stack up those stacks
We are a stack machine, right? I could add another stack for my chunks, and push and pop them 
with individual sizes. Would not even be half bad to manage. But a second stack has implications.
You would need to decide up front, how much space you reserve for either of those. I only one of 
them runs out, your program will fail to complete. So you will always waste memory. Not good. 
Not low overhead.

I do have a stack already. I can just use it from both sides, can I not? That would be a way. 
Makes pushing more difficult (from both ends), but that is okay. In fact, this is what many 
systems with stack and heap do. They have the heap on the upper end of the stack, and they 
grow towards each other. Would this be analog to that?

Not really. It would grow from both ends, but I do not want a heap, if I can avoid it (a heap 
would mean, I have to solve the ref count/garbage collection problem I do not want to have; 
memory fragmentation would also be a potential issue). I want the chunks to live locally, like 
variables (which I only have globally, yet, but you get the point). So I realised, I do not 
want the chunks on the heap, but on the stack, so to speak. Hey - wait a second!

## Everything that exists, exists on the stack
So, put them on the stack! I could have had that earlier in a stack machine, could I not?
Have them live on the stack, in the current execution frame. So they live while deeper function
calls are handled, but are removed (not freed, but popped), when I leave that frame to the 
calling frame. That sounds like efficient usage of memory with no complex overhead for handling 
disposal. Just roll back the frame, and all will be clean.

Of course there is that small detail, that I do not have frames implemented. I wanted to do that 
for "real" function calls inside the VM, but I was a bit worried I will have problems with the 
call convention. I kinda did that for external function calls, but those leave the VM, so there 
is not much to remember (aka store). That is, why I have no local variables, yet. But this is 
a good reason to start on frames.

## Fetch the hammer, bang in those bytes!
Adding a byte-oriented structure to the stack does however does not fit very well with my current 
stack design of having i64 values. I can, of course, start slamming them on the 8 byte sized 
values, padding them to fit in. That would not even be too horrible, would not even need 
`unsafe` code. But it made me think.

## Redesign!
The original idea: having every value 64 bit would make the design clear and simple. But 
experiencing how many values I push to the stack, it works against my main goal: light 
weight. That stack fills by 8 bytes for every value I put there, every variable I 
introduce. And if I would build that byte-chunk-inside-i64-thing I was musing about, 
I lose that simplicity anyway. I think, I at least want to try out working with a 
byte-sized stack. From my gut feeling, I would assume that the complexity added is 
not too bad, if you restrain yourself from becoming too fancy. And you will save a lot 
of ram in VM during execution (which translates to: the stack can be much smaller 
for an equivalent program).


# Bytes will be bytes

Yeah, it is the perfect time for ripping the stack apart and changing it to byte sized values;
the very last thing I did before going to sleep yesterday was adding unit tests for pushing 
and popping stack values. So hurray.

[...]
