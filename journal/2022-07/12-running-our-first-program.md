---
entry: 12
published: 2022-07-12
---

# Running our first program

__Now, that we have a VM, we will run a program on it.__

So we built our very first VM and studied the code in detail. It is time to execute a 
program on it and look at it's output. We will look at every single step the program 
takes. Aren't we lucky, that our VM is so talkative during execution?

If you missed the code, look at the previous post, [A VM](a-vm.md).

## Let's go!

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
There happened quite a lot here. The two values we had before where both popped from the 
stack (so it was shortly empty). The `add` operation adds them, and pushes their sum back 
on the stack. So now there is one value on the stack, and it is the result of our adding 
operation.

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

Go play around with it a bit. It is the best way to learn and to understand. I mean it!
Write a longer program. What happens to the stack? Add another opcode &ndash; how about subtraction?
Will your program execute at all? What happens, if it does not?
