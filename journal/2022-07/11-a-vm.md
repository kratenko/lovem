---
entry: 11
published: 2022-07-11
---

# A VM

__The first draft of source code, that will be our VM, explained.__

I dumped some source code in front of you, and then I started to talk about programming languages. 
Time now, to explain what I did and why. We only have 132 lines, including comments. We will go 
through all parts of it. And I will talk a little about how Rust's basic syntax works, while I 
use it. Not too much, since it is not good Rust code, yet, but to help you start. This will be 
a longer entry.

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

Nothing fancy, just a struct that will represent our Virtual Machine. Only three fields for now:

  1. `stack`: Obviously our stack machine would need one of those. This will hold values during execution. 
     I am using a Vector. That is nothing more than a chunk of memory, that knows how much capacity it has and 
     how many values are in it at the moment. It does support resizing, but I do not want to use that. 
  2. `pc` will be our *program counter*. That is a register [^register] holding the progress in the 
     program during execution. It will always point at the instruction that is to be executed next.
  3. `op_cnt` will be counting the number of operations executed. For now, I want that information 
     out of curiosity, but later it will be useful for limiting execution time for programs.

[^register]: Don't let yourself be confused by fancy terms like *register*. You can think of it as a 
             kind of snobbish variable with a special meaning. In computers sometimes stuff magically
             happens when you write to a *register* &ndash; but it should always be documented somewhere.

`usize` and `i64` are Rust's names for integer types. The language is very explicit in those terms
(and very strict, as in every aspect). I will not give a real introduction to Rust for you (there are 
pages that do that), but I will try to start slowly and give you hints on the important things I 
introduce, so that you get the chance to learn about them parallel to this journal. I hope, that makes it 
easier to follow for Rust beginners. To readers that know Rust: please excuse the crude code here! I will 
make it more rusty, soon. Skip to the next post, if you cannot handle it.

We will also need a program that we will run in our VM. For the start, a crude array of bytes will do. 
The VM will be running bytecode after all. And that really is only that: a bunch of bytes, that you will soon 
be able to understand.

~~~rust
// assign `pgm` to hold a program:
let pgm = [0x00 as u8, 0x01, 100, 0xff];
~~~

We will use a program that is a bit longer, but right now I wanted you to see a program, 
that is actually nothing but a collection of bytes in Rust code. `let` declares and assigns a 
variable here, named `pgm`. It is an array of 4 bytes (`u8` is an unsigned 8bit integer - you might 
know it as `uint8_t` from other languages). And that variable will not be variable at all. By default, 
all variables in Rust are immutable. If you want to change it, later, you would have to declare it  
using the modifier `mut`. 

There is no need to modify the program after creation, we just want to read it for execution. But our 
VM will have to be mutable, as it has changing internal state. Here is our complete `main` function, 
creating the (immutable) program and the (mutable) VM, and running the program. Of course, the `run(...)`
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
We will write some methods for our `VM` struct. Let's start with the one for reading our program:

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

So, what is that `panic!()` you might ask? Well, if we reach that instruction, it will start to 
panic, and then it will die. That is not a nice way to act. Do not worry, we will change that 
to something more reasonable, when we start writing better Rust. And what about the naked `v` 
in the last line? It will have the function return the value of `v`.

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
fetching one instruction at a time and executing it, until we reach the end.
And you might have noticed, that our programm will be very talkative. I added a lot of `println`s, 
that tell just about everything that happens, during execution.

I guess it is time to look at those `op::` constants I keep using.

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
We have 5 different operations for our VM. The only thing missing is some code, that actually executes 
those instructions:

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
Each of our opcodes is handled individually. And we log a lot, so that we can read what is happening, when 
we run it. Ignore the `unwrap()` thingies for the time being. They are just there to try and ignore 
potential runtime errors. Again, not good Rust style, but, you know: later.

The four operations get more complex in what they do. Let's go through them one by one:

  1. `NOP` &ndash; this does nothing, it just wastes bytecode and execution time. I have 
     included it simply to be the most basic operation possible.
  2. `POP` &ndash; this is our first modification of the stack. It simply discards the topmost 
     value, decreasing the stack's size by one.
  3. `PUSH_U8` &ndash; this is the only operation that reads additional data from the program.
     It only reads a single byte (increasing the program counter by one), and puts it on top of 
     the stack, increasing the stack's size by one. This is how you can get data from your program
     into the VM, to work with them. It is how numeric literals in your program are handled.
  4. `ADD` &ndash; the only operation that works on data. It pops its two operands from the stack, 
     adds them, and pushes the sum back on the stack. This is how data is manipulated in a stack 
     machine. The operation reduces the stack's size by one effectively, but there need to be
     at least 2 values on it for it to be executed.

That is the out complete VM so far, and it will execute a program, if you compile and run it
(which we will do in the next post). 

You can find the complete program here: 

https://github.com/kratenko/lovem/blob/v0.0.1-journey/src/main.rs

You can access the repo at this state under (there is also a zip file containing all files):

https://github.com/kratenko/lovem/releases/tag/v0.0.1-journey


## How do I work with the code?
The easy way, to get the code and play with it, would be to clone the git repository and check out 
the tag `v0.0.1-journey`. If you did not understand any of that, you might want to do a tutorial on 
git, before you continue reading. Anyways, here is some copy&paste commands, you can hack into your 
bash prompt, to do, what I just told you to do. Use at your own risk, I'm not responsible for 
what you do to your system.

~~~bash
you@host:~$ git clone https://github.com/kratenko/lovem.git

you@host:~$ cd lovem

you@host:~/lovem$ git checkout v0.0.1-journey

you@host:~/lovam$ cargo run lovem
~~~

This will copy all source code from GitHub and its history to your computer, and it will roll the source code 
to the state we are looking at in this entry. The last command `cargo run lovem` will compile and execute 
the program - that is, if Rust is installed and ready to run (and in the correct version). `cargo` is Rust's 
package manager, that handles dependencies and compiles your projects. I will not explain those things further, 
but now you know what to look for.
