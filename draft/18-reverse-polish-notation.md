---
entry: 18
published: 2022-07-31
tag: v0.0.5-journey
---

# Reverse polish notation

__We are using the design of a stack machine to efficiently execute some calculations.__

The way stack machines work can be used in programs that execute calculations. We will 
look at it by implementing an example from the Wikipedia page about stack machines.

I will quote a lot of it here. You can see the full text of the article and its authors 
when you follow the [Wikipedia permalink to the article][permalink].

> **Design**
> 
> Most or all stack machine instructions assume that operands will be from the stack, 
> and results placed in the stack. The stack easily holds more than two inputs or more than one result, 
> so a rich set of operations can be computed. In stack machine code (sometimes called p-code), 
> instructions will frequently have only an opcode commanding an operation, 
> with no additional fields identifying a constant, register or memory cell, known as a zero address format.[^1] 
> This greatly simplifies instruction decoding. Branches, load immediates, 
> and load/store instructions require an argument field, 
> but stack machines often arrange that the frequent cases of these still fit together with the opcode 
> into a compact group of bits.
> 
> &mdash; <cite>[Wikipedia][permalink] - retrieved 2022-07-15</cite>

So far nothing new - I wrote about all that in my earlier posts.

> The selection of operands from prior results is done implicitly by ordering the instructions. [...]
> 
> &mdash; <cite>[ibid.][permalink]</cite>

Now, here it gets interesting.

> [...]
> 
> The instruction set carries out most ALU actions with postfix ([reverse Polish notation][rev-pol]) 
> operations that work only on the expression stack, not on data registers or main memory cells. 
> This can be very convenient for executing high-level languages, because most arithmetic expressions can be 
> easily translated into postfix notation. 
> 
> For example, consider the expression A*(B-C)+(D+E), written in reverse Polish notation as 
> A&nbsp;B&nbsp;C&nbsp;-&nbsp;*&nbsp;D&nbsp;E&nbsp;+&nbsp;+. 
> Compiling and running this on a simple imaginary stack machine would take the form: 
>
> ~~~
>                 # stack contents (leftmost = top = most recent):
> push A          #           A
> push B          #     B     A
> push C          # C   B     A
> subtract        #     B-C   A
> multiply        #           A*(B-C)
> push D          #     D     A*(B-C)
> push E          # E   D     A*(B-C)
> add             #     D+E   A*(B-C)
> add             #           A*(B-C)+(D+E)
> ~~~
> &mdash; <cite>[ibid.][permalink]</cite>

Well, I don't know about a "simple *imaginary* stack machine" - but as it happens to be, we have a 
very real simple stack machine at our disposal. You know where we will be going next!


## Porting the code to lovem
The program from the [Wikipedia article][permalink] uses 5 variables `A` to `E`. We do not support any kind 
of variables, yet, but that isn't important here. We use immediates (literals from your program) 
to put some concrete values into the calculation. Let's just take some numbers, totally at random:

~~~
A = 5, B = 7, C = 11, D = 13, E = 17
~~~

And we add a new binary to the project: [`reverse-polish.rs`][polish]

~~~rust
//! A small program demonstrating execution of arithmetics in our VM.
//!
//! For an explanation of what we are doing here, look at this wikipedia article:
//! https://en.wikipedia.org/w/index.php?title=Stack_machine&oldid=1097292883#Design
use lovem::{op, VM};

// A*(B-C)+(D+E)
// A B C - * D E + +
// A = 5, B = 7, C = 11, D = 13, E = 17
// 5 * (7 - 11) + (13 + 17) = 10

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::PUSH_U8, 5, op::PUSH_U8, 7, op::PUSH_U8, 11, op::SUB, op::MUL,
        op::PUSH_U8, 13, op::PUSH_U8, 17, op::ADD, op::ADD, op::POP, op::FIN];
    // Create our VM instance.
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

The comments spoil the result, but we want to check it calculates correctly, so that is okay. The 
program is the same as before: create a VM and run some hardcoded bytecode on it. Since the VM
logs excessively, we will see what happens, when we run it. So the only new thing here is the 
bytecode program. I'll write it down in a more readable form:

~~~
push_u8 5
push_u8 7
push_u8 11
sub
mul
push_u8 13
push_u8 17
add
add
pop
fin
~~~

To no-ones surprise, this code is the same as in the article - only with the variables replaced by 
numbers, and I added a `pop` and a `fin` at the end, to keep our program clean.

## Execution
~~~
VM { stack: [], pc: 0, op_cnt: 0 }
Executing op 0x02
  PUSH_U8
  value: 5
VM { stack: [5], pc: 2, op_cnt: 1 }
Executing op 0x02
  PUSH_U8
  value: 7
VM { stack: [5, 7], pc: 4, op_cnt: 2 }
Executing op 0x02
  PUSH_U8
  value: 11
VM { stack: [5, 7, 11], pc: 6, op_cnt: 3 }
Executing op 0x11
  SUB
VM { stack: [5, -4], pc: 7, op_cnt: 4 }
Executing op 0x12
  MUL
VM { stack: [-20], pc: 8, op_cnt: 5 }
Executing op 0x02
  PUSH_U8
  value: 13
VM { stack: [-20, 13], pc: 10, op_cnt: 6 }
Executing op 0x02
  PUSH_U8
  value: 17
VM { stack: [-20, 13, 17], pc: 12, op_cnt: 7 }
Executing op 0x10
  ADD
VM { stack: [-20, 30], pc: 13, op_cnt: 8 }
Executing op 0x10
  ADD
VM { stack: [10], pc: 14, op_cnt: 9 }
Executing op 0x01
  POP
  dropping value 10
VM { stack: [], pc: 15, op_cnt: 10 }
Terminated!
VM { stack: [], pc: 16, op_cnt: 11 }
Execution successful.
~~~

The output shows you the stack after every instruction. You can compare it to the 
stack contents in the Wikipedia listing, and you will find them identical (the order 
of the stack listing is switched, and of course you have numbers instead of arithmetic
expressions with variables &ndash; but if you insert our numbers on the Wikipedia 
listing they should match).

Our PoC stack machine really can do what the imaginary one is claimed to do. That's nice.

## Homework
You should really read the article on [Reverse Polish Notation][rev-pol] 
*([permalink to article at time of writing][rev-pol-perma])*. It will give some background on why it 
is important, not at least historically. The Z3, for example, arguably the first computer built by 
mankind[^2], was using it.

[permalink]: https://en.wikipedia.org/w/index.php?title=Stack_machine&oldid=1097292883#Design
[rev-pol]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
[rev-pol-perma]: https://en.wikipedia.org/w/index.php?title=Reverse_Polish_notation&oldid=1096284746
[polish]: https://github.com/kratenko/lovem/blob/v0.0.5-journey/src/bin/reverse-polish.rs
[^1]: Beard, Bob (Autumn 1997). ["The KDF9 Computer - 30 Years On"](http://www.cs.man.ac.uk/CCS/res/res18.htm#c). Computer RESURRECTION.
[^2]: Yeah, I know. The answer to the question "What was the first machine to qualify as a computer?", differs, 
depending on whom you ask &ndash; and also on the country you ask the question in. But the Z3 is 
a prominent candidate.