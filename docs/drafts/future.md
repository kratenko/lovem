# Let there be source code
I have written code. And this time, I (re-)started lovem in a public git repository, 
so you can see what I do, if you are interested. And I hope it puts enough pressure
on me, to keep on the project for a while.

In fact, there is quite a bit of code there already. I started coding, before writing 
any of this, and it when so well. I like how it feels. I was working any hour I 
could spare. When a friend asked me what I was doing, I started a somewhat 
complex backstory why I was doing it, instead of actually explaining anything of the 
stuff I was doing &ndash; and was interrupted quite early, so there was more to tell in me still. 
The next day, I sat down and started to write that down as a little story. I wanted to 
put it somewhere, so I started this journal to publish it. And I decided to do it in 
blog form, so I am publishing that background story bit by bit.

So, as of writing this, there is a lot of work completed on the VM. Tt is amazing what 
things it can do for how little code there is. When this post goes public, there should be 
quite lot more done...

## But where is the code?
Well, if you read this journal, you will know where it lives. Anyway, this is the repo:

https://kratenko.github.io/lovem/

I plan to continue sharing my thoughts while I work on the VM. So you will be able to 
follow my failures and see the attempts that I will be ditching later. I think the 
format of this journal can work out, but we will see how I like it over time. It will be 
behind on progress, as I want to take time to share things as they unfold. And this should 
help to produce a somewhat continuous publication stream. Git being what git is, should support
me in showing you the things I do back in time, using the power of commits.

As things are with blogs, my entries will be very different, depending on what I want to tell 
and on what I did. So far most blogs where conceptional thinking, some research, and a lot of 
blabla, which I tell because it interests me myself. In the future, there should be concrete 
problems I find and solve in source code - or which I fail to solve.

## Back in time
Time will have advanced, when you read this. But you can find my initial commit here: 

https://github.com/kratenko/lovem/tree/c8bfa05b8d3276241ae9d2c46f756ec7f1333af0

I think this will be a theme in this journal, linking you to what I did, when I am writing about it.
I notice now, that I did my first commit way too late, so there will be a lot to talk about for that 
commit alone. So it will be then.

The next journal entry will be about some decisions again, mainly about the language I'll be using.



# It looks so weird
So, did you take a look at the code, yet? In case you've forgotten, this is my initial commit:

https://github.com/kratenko/lovem/tree/c8bfa05b8d3276241ae9d2c46f756ec7f1333af0

A lot of code for an initial commit, I know - but it was going so well, so I guess I forgot... and 
the idea of this journey came later. But I digress.

If you are thinking: "What is that weird source code?", then you are in for a real treat (and a lot of pain), 
should you chose to follow up. The code you are seeing is written in [Rust][rust].

## Once again: but why?
Why Rust? Because Rust! Writing Rust can feel so good! And for something like a VM, it is such a good choice. 
If you have never heard of the language (or heard of it, but never looked at it), it is hard to understand 
why this is so. My advice: try it! use it! Or read along this journal, code along, you might like it.

When you start, you will *not* like Rust. The compiler is a pedantic pain in the ass. But at the same time 
it is incredibly polite. And rust really, really tries, to keep you from shooting yourself in the foot. It 
tries to make common mistakes impossible or at least hard to do &ndash; those mistakes that happen everywhere 
in C/C++ programs and their like. Yes, those mistakes that are the cause of the majority of all security 
problems and crashes. Buffer overruns, use after free, double free, memory leak &ndash; to name just some 
common once from the top of my head. And Rust makes all it can to make those mistakes impossible *during 
compilation!* So it does not even add runtime overhead. That is so powerful!

And it is so painful. Half of the things you do, when writing C/C++, you will not be able to do in Rust 
in the same way. Every piece of memory is owned. You can borrow it and return it, but it cannot be owned 
in two places at once. And if any part of the program has writing access to it, no other part may have 
any access. This makes some data structures complicated or impossible (there are ways around it), and you 
will have to thing quite differently. But if you give in on that way of thinking, you can gain so much. 
Even peace of the mind, as the coding world will look a lot saner inside Rust source code. This will, of 
course, come with the price, that all code in other languages will start to feel dirty to you, but that 
is the way.

Also, there are a lot of ways to write code, that you cannot add to a language that already exists. 
C and C++ will never be freed of their heritage; they will stay what they are, with all their pros 
and cons. Things are solved differently. Did I mention there is no `NULL`? And I have never missed 
it for a moment. Rust solves the problems other languages solve with `NULL` by using enums. That comes 
with certainty and safety all the way. There are no exceptions either. That problem is also solved 
by using enums. The way the language embraces those, they are a really powerful feature! And there are 
lot more convenient ways of organising code, that I keep missing in my daily C/C++ life.

This post will not be an introduction into Rust (but this journal might become one, not the basics, but
a usage example to learn by). I just wanted 
to share some of the love that the language gives, to try and give you the courage to embrace Rust. 

## Didn't you say, you use C/C++?
Yes I did say that. And I do use those. It is not easy to change that, when you have a certain amount 
of legacy code (and not much experience with the new language, as we do not really have, yet). But we 
do have a saying these days. Often, after an hours long debugging session, when we find the bug, 
understand it and fix it, there is this realisation, that fit in the sentence:

"Mit Rust wär das nicht passiert." &mdash; "This would not have happened with Rust."

So, this will not happen to me with this project, because those things will not happen with Rust!

[rust]: [https://rust-lang.org]



# A VM
I did commit way to late, so I had to go back and start my first simplistic version again from 
memory. I will have to develop it again, as well as I can, to let you in on it. We will do that 
in the branch `the-beginning`, so we have it documented. I'm sorry for that, but I did not know 
I would write this journal back then. I will be writing it more clearly and better to follow
this time.

## I swear, if I do not see some code in this post...
Alright, alright... We will start with our VM:

~~~rust
#[derive(Debug)]
pub struct VM {
    stack: Vec<i64>,
    pc: usize,
    op_cnt: usize,
}
~~~

Nothing fancy, just a struct that will represent our virtual machine. Just three fields for now:

  1. `stack`: Obviously our stack machine would need one of those. This will hold values during execution. 
     I am using a Vector. That is nothing more than a chunk of memory, that knows how much capacity it has and 
     how many values are in it at the moment. It does support resizing, but I do not want to use that. 
  2. `pc` will be our *program counter*. That is a register [^register] holding the progress in the 
     program during execution. It will point at the instruction that is to be executed next.
  3. `op_cnt` will be counting the number of operations executed. For now, I want that information 
     out of curiosity, but later it will be useful for limiting execution time for programs.

[^register]: Don't let yourself be confused by fancy terms like "register". You can think of it as a 
             kind of snobbish variable with a special meaning. In computers sometimes things magically
             happen when you write to a "register" &ndash; but it should always be documented somewhere.

`usize` and `i64` are Rust's names for integer types. The language is very explicit in those terms
(and very strict, as in every aspect). I will not give a real introduction to Rust for you (there are 
pages that do that), but I will try to start slowly and give you hints on the important things I 
introduce, so that you get the chance to learn about them parallel to this journal. I hope, that makes it 
easier to follow for Rust beginners. To readers that know Rust: please excuse the crude code here! I will 
make it more rusty, soon. Skip to the next post, if you cannot handle it.

We will also need a program that we will run in our VM. For the start, a crude array of bytes will do. 
The VM will be running bytecode after all. And that really is that: a bunch of bytes, that you will soon 
be able to understand.

~~~rust
// assign `pgm` to hold a program:
let pgm = [0x00 as u8, 0x01, 100, 0xff];
~~~

We will use a longer program, that is a bit longer, but right now I wanted you to see a program, 
that is actually nothing but a collection of bytes in Rust code. `let` declares and assigns a 
variable here, named `pgm`, that is an array of 4 bytes (`u8` is an unsigned 8bit integer - you might 
know it as `uint8_t` from other languages). And that variable will not be variable at all. By default, 
all variables in Rust are immutable. If you want to change it, later, you would have to declare it like 
using the modifier `mut`. 

There is no need to modify the program after creation, we just want to read it for execution. But our 
VM will have to be mutable, as it has changing internal state. Here is our complete `main` function, 
creating the (immutable) program, the (mutable) VM, and running the program. Of course, the `run(...)`
method is still missing. And you will see the program, we will be using (with some constants that I
did not define, yet).

~~~rust
fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::NOP, op::PUSH_U8, 100, op::PUSH_U8, 77, op::ADD, op::POP, 0xff];
    // Crate our VM instance.
    let mut vm = VM{
        stack: Vec::with_capacity(100),
        pc: 0,
        op_cnt: 0
    };
    // Execute the program in our VM:
    vm.run(&pgm);
}
~~~


## Behaviour for our VM
So far we only have an initialized data structure and some bytes. Let's do something with it. 
Rust does not really use objects (and I think that is good). 
But it has *associated functions* that work on types, and *methods* that work on instances of types. 
We will write some methods for our VM struct. Let's start with the one for reading our program:

~~~rust
impl VM {
    /// Fetch the next byte from the bytecode, increase programm counter, and return value.
    fn fetch_u8(&mut self, pgm: &[u8]) -> u8 {
        if self.pc >= pgm.len() {
            panic!("End of program exceeded");
        }
        let v = pgm[self.pc];
        self.pc += 1;
        v
    }
}
~~~

The `fetch` method will work on our VM instance. The first parameter is `&mut self` &ndash; that
tells us it works on an instance of the type `VM`. It will work on a reference to the instance
(indicated by the `&`), and it can modify the data (indicated by the `mut`). It will also take the 
reference to an array of `u8`s, but that it will not be able to modify (no `mut`). It returns a `u8`.

What it does is simply read and return a byte from the program, and increase the VMs internal 
program counter by one, so that the next call to `fetch` will return the next byte. Simple. 

Now, let's look at that `run` method, we were calling in `main`:

~~~rust
impl VM {
    /// Executes a program (encoded in bytecode).
    pub fn run(&mut self, pgm: &[u8]) {
        // initialise the VM to be in a clean start state:
        self.stack.clear();
        self.pc = 0;
        self.op_cnt = 0;

        // Loop going through the whole program, one instruction at a time.
        loop {
            // Log the vm's complete state, so we can follow what happens in console:
            println!("{:?}", self);
            // Fetch next opcode from program (increases program counter):
            let opcode = self.fetch_u8(pgm);
            // We count the number of instructions we execute:
            self.op_cnt += 1;
            // If we are done, break loop and stop execution:
            if opcode == op::FIN {
                break;
            }
            // Execute the current instruction (with the opcode we loaded already):
            self.execute_op(pgm, opcode);
        }
        // Execution terminated. Output the final state of the VM:
        println!("Terminated!");
        println!("{:?}", self);
    }
}
~~~

The comments should explain, what is going on there. Initialise VM, then loop over the program, 
fetching one instruction at a time and executing it, until we reach the end. I guess it is time 
to look at those `op::` constants I keep using.

~~~rust
/// Module holding the constants defining the opcodes for the VM.
pub mod op {
    /// opcode: Do nothing. No oparg.
    ///
    /// pop: 0, push: 0
    /// oparg: 0
    pub const NOP: u8 = 0x00;
    /// opcode: Pop value from stack and discard it.
    ///
    /// pop: 1, push: 0
    /// oparg: 0
    pub const POP: u8 = 0x01;
    /// opcode: Push immediate value to stack.
    ///
    /// pop: 0, push: 1
    /// oparg: 1B, u8 value to push
    pub const PUSH_U8: u8 = 0x02;
    /// opcode: Add top two values on stack.
    ///
    /// pop: 2, push: 1
    /// oparg: 0
    pub const ADD: u8 = 0x10;
    /// opcode: Terminate program.
    ///
    /// pop: 0, push: 0
    /// oparg: 0
    pub const FIN: u8 = 0xff;
}
~~~

Just 5 `u8` constants there, grouped in a module as a namespace. And a lot of comments to explain them.
We have 5 different operations for our VM. The only thing missing is to teach it how to execute those:

~~~rust
impl VM {
    /// Executes an instruction, using the opcode passed.
    ///
    /// This might load more data from the program (opargs) and
    /// manipulate the stack (push, pop).
    fn execute_op(&mut self, pgm: &[u8], opcode: u8) {
        println!("Executing op 0x{:02x}", opcode);
        match opcode {
            op::NOP => {
                println!("  NOP");
                // do nothing
            },
            op::POP => {
                println!("  POP");
                let v = self.stack.pop().unwrap();
                println!("  dropping value {}", v);
            },
            op::PUSH_U8 => {
                println!("  PUSH_U8");
                let v = self.fetch_u8(pgm);
                println!("  value: {}", v);
                self.stack.push(v as i64);
            },
            op::ADD => {
                println!("  ADD");
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b);
            },
            _ => {
                panic!("unknown opcode!");
            }
        }
    }
}
~~~

You can think of the `match` as a switch statement. It is much more than that, but here we use it as one.
Each of our opcodes is handles individually. And we log a lot, so that we can read what is happening, when 
we run it.

The four operations get more complex in what they do. Let's go through them one by one:

  1. `NOP` &ndash; this does nothing, it just wastes bytecode and execution time. I have 
     included it simply to be the most basic operation possible.
  2. `POP` &ndash; this is our first modification of the stack. It simply discards the topmost 
     value, decreasing the stack's size by one.
  3. `PUSH_U8` &ndash; this is the only operation that reads additional data from the program.
     It only reads a single byte (increasing the program counter by one), and puts it on top of 
     the stack, increasing the stack's size by one. This is how you can get data from your program
     into the VM, to work with them.
  4. `ADD` &ndash; the only operation that works on data. It pops its two operands from the stack, 
     adds them, and pushes the sum back on the stack. This is how data is manipulated in a stack 
     machine. The operation reduces the stack's size by one effectively, but there need to be
     at least 2 values on it for it to be executed.

That is the out complete VM so far, and it will execute a program, if you compile and run it
(which we will do in the next post). 

You can find the complete program here: 

https://github.com/kratenko/lovem/tree/db9691e782e13efd449a0079c081ce6cc5e96a28



# Running our first program

So we built our very first VM. You can find it here:

https://github.com/kratenko/lovem/tree/db9691e782e13efd449a0079c081ce6cc5e96a28

Let's go!

~~~
/home/kratenko/.cargo/bin/cargo run --color=always --package lovem --bin lovem
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/lovem`
VM { stack: [], pc: 0, op_cnt: 0 }
Executing op 0x00
  NOP
VM { stack: [], pc: 1, op_cnt: 1 }
Executing op 0x02
  PUSH_U8
  value: 100
VM { stack: [100], pc: 3, op_cnt: 2 }
Executing op 0x02
  PUSH_U8
  value: 77
VM { stack: [100, 77], pc: 5, op_cnt: 3 }
Executing op 0x10
  ADD
VM { stack: [177], pc: 6, op_cnt: 4 }
Executing op 0x01
  POP
  dropping value 177
VM { stack: [], pc: 7, op_cnt: 5 }
Terminated!
VM { stack: [], pc: 8, op_cnt: 6 }

Process finished with exit code 0
~~~

## What just happened?

It is quite talkative. And isn't it nice, how easy it is, to print the complete state of our VM in Rust?
And it costs no overhead during runtime, as it is generated during compilation for us. Isn't that something?

So, what is happening there? Our program `pgm` looks like this:

~~~rust
    let pgm = [op::NOP, op::PUSH_U8, 100, op::PUSH_U8, 77, op::ADD, op::POP, 0xff];
~~~

That are 8 bytes that consist of 6 instructions. Each instruction has a 1 byte opcode.
Two of those instructions (the `PUSH_U8`) have one byte of argument each, 
making up the remaining two bytes of our program. Here they are listed:

  1. `NOP`
  2. `PUSH_U8 [100]`
  3. `PUSH_U8 [77]`
  4. `ADD`
  5. `POP`
  6. `FIN`

The `NOP` does not do anything. I just put it in front of the program to let you see
fetching, decoding, and executing without any effects:

~~~
VM { stack: [], pc: 0, op_cnt: 0 }
Executing op 0x00
  NOP
VM { stack: [], pc: 1, op_cnt: 1 }
~~~

We just increased the program counter by one (we advance one byte in the bytecode), and the 
operation counter counts this executed instruction. Let's look at the next instruction, that is more
interesting:

~~~
VM { stack: [], pc: 1, op_cnt: 1 }
Executing op 0x02
  PUSH_U8
  value: 100
VM { stack: [100], pc: 3, op_cnt: 2 }
~~~

Here the PC is increased by two. That happens, because we fetch an additional value from the bytecode.
The op_cnt is only increased by one. And we now have our first value on the stack! It is the byte 
we read from the bytecode. Let's do that again:

~~~
VM { stack: [100], pc: 3, op_cnt: 2 }
Executing op 0x02
  PUSH_U8
  value: 77
VM { stack: [100, 77], pc: 5, op_cnt: 3 }
~~~

Now there are two values on the stack! Time to do something with them. Let's add them up:

~~~
VM { stack: [100, 77], pc: 5, op_cnt: 3 }
Executing op 0x10
  ADD
VM { stack: [177], pc: 6, op_cnt: 4 }
~~~

Now there is only one value left on the stack, and it is the sum of the two values we had.
What's next?

~~~
VM { stack: [177], pc: 6, op_cnt: 4 }
Executing op 0x01
  POP
  dropping value 177
VM { stack: [], pc: 7, op_cnt: 5 }
~~~

It is always nice to leave your workplace all tidied up, when you are done. We can do that by 
popping our result back from the stack, leaving it empty. And besides, our `POP` operation 
prints the value it drops. One more instruction to go:

~~~
VM { stack: [], pc: 7, op_cnt: 5 }
Terminated!
VM { stack: [], pc: 8, op_cnt: 6 }
~~~

Well, not much happening there. Just stopping the VM, because we are done.

## Success!
So, we ran a program in a VM. Hooray, we are done. Only 132 lines of code, including excessive 
comments and logging. That was easy.

Well yeah - it doesn't do much. But you can understand the root principle that makes up a 
stack machine. It's that simple. 

Go play around with it a bit. It is the best way to learn and to understand.
Write a longer program. Add another opcode &ndash; how about subtraction?



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
