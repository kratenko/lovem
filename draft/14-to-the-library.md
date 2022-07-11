---
entry: 14
published: 2022-07-20
tag: v0.0.3-journey
---


# To the library!

__We turn our project from a binary project into a library project.__

So far, our lovem cargo project holds a single binary. That is not very useful for something that should be 
integrated into other projects. What we need is a *library*. How is that done? Simple: we rename our 
`main.rs` to `lib.rs`.

## No main?
But wait? What about `fn main()`? We do not need that inside a library. But it would be nice to still 
have some code that we can execute, right? Well, no problem. Your cargo project can only hold a single 
library, but it can hold even multiple binaries, each with its own `fn main()`. Just stuff them in the 
`bin` subdir. 

## Project layout
While we are at it, I split the project up into multiple source files, to get it organised. It is small, still, 
but we will have it grow, soon. Here is, what we are at now:

~~~
lovem/
  src/
    bin/
      test-run.rs
    lib.rs
    op.rs
    vm.rs
  .gitignore
  Cargo.toml
~~~

We skip `.gitignore`. If you don't know what it is, [google `.gitignore`](https://www.google.com/search?q=.gitignore).

### Cargo.toml
So `Cargo.toml` holds information about our cargo project. There is not much of interest there currently:

~~~toml
[package]
name = "lovem"
version = "0.0.3"
edition = "2021"
authors = ["kratenko"]

[dependencies]
~~~

The only real configuration in that file is `edition = "2021"`. Rust has a mager edition release every three years. 
These are used to introduce braking changes. You have to specify the edition you use explicitly, and there are 
migration guides. We use the most recent one, `2021`.

### lib.rs
Rust manages projects by using default project layouts. That is why we need not write a lot into the `Cargo.toml`.
The `src` directory holds our source code. The fact that it holds a `lib.rs` makes it a library, and `lib.rs` is 
the entry point. This is what is in it:

~~~rust
pub mod op;
pub mod vm;

// re-export main types
pub use crate::vm::VM;
~~~

Really not a lot. It declares the two modules `op` and `vm` and makes them public. So, whatever rust project 
will be using our library will have access to those modules. The modules will be in the files `op.rs` and 
`vm.rs`. What a coincidence, that are exactly the remaining two source files in this directory!

The last line just re-exports a symbol from one of those submodules, so that programs using our 
library can access more easily. Will will be doing that in our binary.

### op.rs
Back in [`v0.0.2-journey`](https://github.com/kratenko/lovem/blob/v0.0.2-journey/src/main.rs), 
we already had a module called `op` to hold the opcodes. We had it stuffed in our `main.rs`. Now it 
lives in a separate file, so we do not have to scroll over it every time.

### vm.rs
This holds the rest of our source code (except for `fn main()` which has no place in a lib). The only 
new thing, compared with our former `main.rs` is the first line:

~~~rust
use crate::op;
~~~

This simply pulls the module `op` into the namespace of this module, so that we can access our 
opcode constants as we did before. The rest remains the way we already know.

### bin/test-run.rs
So how do we use our lib in a project? That is best illustrated by doing it. And we can do so inside 
our project itself, because we can add binaries. Just put a Rust source file with a `fn main()`
inside the `bin` subdir. There we can write a binary as we would in a separate project, that can use 
the lib.

We did that in the file `test-run.rs`:

~~~rust
use lovem::{op, VM};

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::NOP, op::PUSH_U8, 100, op::PUSH_U8, 77, op::ADD, op::POP, 0xff];
    // Crate our VM instance.
    let mut vm = VM::new(100);
    // Execute the program in our VM:
    match vm.run(&pgm) {
        Ok(_) => {
            println!("Execution successful.")
        }
        Err(e) => {
            println!("Error during execution: {:?}", e);
        }
    }
}
~~~

This is the `fn main()` function from our former `main.rs`. Instead of having all the functions 
and definitions, it just has this single line at the top: 

~~~rust
use lovem::{op, VM};
~~~

Nothing too complicated. It tells the compiler, that our program uses the library called `lovem`
(which is, of course, the one we are writing ourselves here). It also tells it to bring the two 
symbols `op` and `VM` from it into our namespace.

The `op` one is simply the module `op` defined in `op.rs`. Because `lib.rs` declares the module 
public, we can access it from here. `VM` does not refer to the module in `vm.rs`, as that module 
is called `vm` (in lower case). `VM` is actually the struct we defined in `vm`, that we use to 
hold the state of our Virtual Machine.

We could include the struct as `lovem::vm::VM`, which is its full path. But I find that a bit 
anoying, as `VM` is the main type of our whole library. We will always be using that. So I 
re-exported it in `lib.rs`. Remember the line `pub use crate::vm::VM;`? That's what it did.

## Running the binary
So, how do we run our program now? Back in `v0.0.2-journey` we simply called
`cargo run`. That actually still works, as long as we have exactly one binary.

But we can have multiple binaries inside our project. If we do, we need to tell cargo which it should run. That 
can easily be done:

~~~shell
cargo run --bin test-run
~~~

The parameter to `--bin` is the name of the file inside `bin`, without the `.rs`. And no configuration 
is needed anywhere, it works by convention of project layout. 

## Homework
What, homework again? Yeah, why not. If it fits, I might keep adding ideas for you to play around with.
Doing things yourself is understanding. Stuff we just read, we tend to forget. So here is what might help 
you understand the project layout stuff I was writing about:

Add a second binary, that runs a different program in the VM (with different bytecode). 
You have all the knowledge to do so. And then run it with cargo. 

## Source code
In earlier posts I included explicit links to the source code at the time of writing. That got 
annoying to do really fast. So I added a new feature to my `blogem.py` that I use to write this journal. 
Entries like this, that are explaining a specific state of the source of lovem will have a *tag* from 
now on. This corresponds to a tag inside the git repository, as it did in earlier posts. You will 
find it in the card at the top of the post (where you see the publishing date and the author). It is 
prefixed with a little tag image. For this post it looks like this:

:octicons-tag-24: [v0.0.3-journey](https://github.com/kratenko/lovem/releases/tag/v0.0.3-journey)

At the bottom of the entry (if you view it in the entry page, not in the "whole month" page), you will 
find it again with a list of links that help you access the source in different ways. The best way to 
work with the code, is to clone the repository and simply check out the tag. I also added a page on this site, 
explaining how you do that. You can find it under [Source Code](../source-code.md). 

So, in future I will not be adding explicit links, only this implicit ones. And there will be a 
link to the explaining page at the bottom. This should be convenient for both, you and me.