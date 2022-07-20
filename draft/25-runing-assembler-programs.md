---
entry: 25
published: 2022-07-31
tag: v0.0.9-journey
---

# Running assembler programs

__We will extend our assembler to do something useful, finally: execute our programs on lovem.__

We have created ourselves an assembler in ~300 lines of code. And it has a command line interface, 
an API to be used in a program, and even useful error reporting. That is cool! But what do we do 
with the bytecode? It just dumps them to the console. That is not very useful. We could copy/paste 
that into one of our example binaries... This is not what we wanted. So let us enhance our assembler.

## Execution
We add some features to 
[`lovas.rs`](https://github.com/kratenko/lovem/blob/v0.0.9-journey/src/bin/lovas.rs). 
A new command line parameter `--run`, that takes no arguments. If you add that flag to the call, 
`lovas` will take the assembled program (if there are no errors), create an instance of the 
VM and run the program on it. Thanks to clap, that is really easy to do. We add another field 
to our `Cli` struct. Actually, while we are at it, we add four new parameters:

~~~rust
#[clap(short, long, help = "Run the assembled program in lovem.")]
run: bool,

#[clap(long, help = "Enable tracing log when running lovem.")]
trace: bool,

#[clap(long, help = "Output the program to stdout.")]
print: bool,

#[clap(long, default_value_t = 100, help = "Setting the stack size for lovem when running the program.")]
stack_size: usize,
~~~

And we change what we do with a successfully created program, depending on our new flag:

~~~rust
// run the assembler:
match asm::assemble(&name, &content) {
    Ok(pgm) => {
        if args.print {
            println!("{:?}", pgm);
        }
        // we succeeded and now have a program with bytecode:
        if args.run {
            // lovas was called with `--run`, so create a VM and execute program:
            run(&pgm, &args)?
        }
        Ok(())
    },
    Err(e) => {
        // Something went wrong during assembly.
        // Convert the error report, so that `anyhow` can do its magic
        // and display some helpful error message:
        Err(Error::from(e))
    },
}
~~~

Just printing the program to stdout is no very useful default behaviour for an assembler.
It might still come in handy, if you want to see what you are executing, so we make it 
optional and for the caller to decide with the `--print` flag.
If the `--run` flag is set, we call `run()`. So what does `run()` do?

~~~rust
/// Executes a program in a freshly created lovem VM.
fn run(pgm: &Pgm, args: &Cli) -> Result<()> {
    // Create our VM instance.
    let mut vm = VM::new(args.stack_size);
    vm.trace = args.trace;
    let start = Instant::now();
    let outcome = vm.run(&pgm.text);
    let duration = start.elapsed();
    match outcome {
        Ok(_) => {
            // Execution successful, program terminated:
            eprintln!("Terminated.\nRuntime={:?}\nop_cnt={}, pc={}, stack-depth={}, watermark={}",
                      duration,
                      vm.op_cnt, vm.pc, vm.stack.len(), vm.watermark
            );
            Ok(())
        },
        Err(e) => {
            // Runtime error. Error will be printed on return of main.
            eprintln!("Runtime error!\nRuntime={:?}\nop_cnt={}, pc={}, stack-depth={}, watermark={}",
                      duration, vm.op_cnt, vm.pc, vm.stack.len(), vm.watermark);
            Err(Error::from(e))
        }
    }
}
~~~

We create a VM instance, and we run the program on it. If there is a `RuntimeError`, we return it, 
just as we did with the `AsmErrorReport`. Back in our examples, we created a VM with a stack size 
of `100` - simply because we needed a number there. `100` is still the default, but now you can 
choose the stack size, when calling `lovas`. If you do 

    lovas --run pgm/some-program.lva --stack-size 512

lovas will execute the program in a VM with a stack that can hold 512 values.

## Trace Log
When we were running a program in our VM, we did always get a lot of output during execution.
That is nice for understanding, what a stack machine does, but in general it is not a got idea 
for a VM to do that. It can be very beneficial, if you run into a problem with your program, 
so it is an easily available tool for debugging. That is why I removed all those log messages 
from lovem, but I let some in that can be activated, if you set `vm.trace = true`. That is what 
we added the new command line parameter `--trace` for. You can now control, if you want to 
see it.

## Diagnostics
There is some output by `lovas`, after the execution. It reports if the run was successfully 
terminated (by executing a `fin` instruction), or if there was a `RuntimeError`. In both 
cases it will show you the time the execution took (wallclock time), as well as the number 
of instructions executed by the VM, the final position of the programm counter, the number of 
values on the stack at termination, and the highest number of values on the stack at any 
time during execution (the watermark). This can give you some quick insight on what your 
program did and maybe where it ran into trouble.

All this lead to some changes to `vm.rs`, but nothing that should give you any 
problems to understand. Remember that we have the power of git at our disposal, so 
you can easily find out what changed in a file between two releases. You could do that for 
`vm.rs` with this handy link:

https://github.com/kratenko/lovem/compare/v0.0.8-journey...v0.0.9-journey#diff-3bc51552cab41d1a2dbf07842cb438088563f6134a9c69a266dfd0d79b631495

## Our programs
We have written a few example programs so far. Each is its own binary in `src/bin/`, and 
all of them consist of the same Rust code of creating a VM and running a program. Only the 
bytecode changed between them. 

I got rid of all of those (except for the most basic one) and translated the programs into 
assembly programs that live in `pgm/`. You can now execute those using `lovas`, like this:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/reverse-polish.lva --trace

   Compiling lovem v0.0.9 (/home/kratenko/git/lovem)
    Finished dev [unoptimized + debuginfo] target(s) in 2.02s
     Running `target/debug/lovas -r pgm/reverse-polish.lva --trace`
VM { stack: [], pc: 0, op_cnt: 0, trace: true, watermark: 0 }
Executing op 0x02
VM { stack: [5], pc: 2, op_cnt: 1, trace: true, watermark: 1 }
Executing op 0x02
VM { stack: [5, 7], pc: 4, op_cnt: 2, trace: true, watermark: 2 }
Executing op 0x02
VM { stack: [5, 7, 11], pc: 6, op_cnt: 3, trace: true, watermark: 3 }
Executing op 0x11
VM { stack: [5, -4], pc: 7, op_cnt: 4, trace: true, watermark: 3 }
Executing op 0x12
VM { stack: [-20], pc: 8, op_cnt: 5, trace: true, watermark: 3 }
Executing op 0x02
VM { stack: [-20, 13], pc: 10, op_cnt: 6, trace: true, watermark: 3 }
Executing op 0x02
VM { stack: [-20, 13, 17], pc: 12, op_cnt: 7, trace: true, watermark: 3 }
Executing op 0x10
VM { stack: [-20, 30], pc: 13, op_cnt: 8, trace: true, watermark: 3 }
Executing op 0x10
VM { stack: [10], pc: 14, op_cnt: 9, trace: true, watermark: 3 }
Executing op 0x01
VM { stack: [], pc: 15, op_cnt: 10, trace: true, watermark: 3 }
Terminated!
VM { stack: [], pc: 16, op_cnt: 11, trace: true, watermark: 3 }
Terminated.
Runtime=49.33µs
op_cnt=11, pc=16, stack size=0, watermark=3
~~~

Remember to add `--trace` to the call, or you won't see very much. It has become a lot easier, 
to play around with the VM. No more writing bytecode by hand!

## File extension
You might have noticed that I changed the filename extension that I use for the assembly programs 
from `.lass` to `.lva`. There are multiple reasons, but the main one is, that I thought *Lass* 
could be a nice name for a programming language, when I will finally come to writing one for 
lovem. So I want to reserve the extension for that possible future.

## Playing around
The diagnostic information given after the execution can be interesting, when you mess around. 
Let us play a bit with the program 
[`endless-stack.lva`](https://github.com/kratenko/lovem/blob/v0.0.9-journey/pgm/endless-stack.lva).

~~~
# This program runs in an endless loop, but it will push a new value to the stack on every iteration.
# It will inevitably lead to a stack overrun at some point and crash the program.
push_u8 123
goto -5
fin
~~~

The program will fill the stack until it is full, and then it will crash:

~~~
     Running `target/debug/lovas -r pgm/endless-stack.lva --print`
Pgm { name: "pgm/endless-stack.lva", text: [2, 123, 32, 255, 251, 255] }
Runtime error!
Runtime=41.589µs
op_cnt=201, pc=2, stack-depth=100, watermark=100
Error: StackOverflow
~~~

After 201 executed instructions it crashes. The stack depth at the time of the crash is 100. That is 
the complete stack, the next instruction tried to push value 101, which must fail. 
Instruction number 201 did cause the crash. That makes sense, if you follow the execution 
in your head. And the program counter is on 2. The last instruction executed will be the 
one before that, which would be at 0. That is the `push_u8` instruction. There is no 
surprise that the watermark is at 100. That is the highest possible value for it and also 
the current value of out stack depth.

As we can now easily change the stack size, let us try what happens with a bigger stack:

~~~
     Running `target/debug/lovas -r pgm/endless-stack.lva --print --stack-size 150`
Pgm { name: "pgm/endless-stack.lva", text: [2, 123, 32, 255, 251, 255] }
Runtime error!
Runtime=47.648µs
op_cnt=301, pc=2, stack-depth=150, watermark=150
Error: StackOverflow
~~~

So now the stack overflows at over 150 values, of course. And it takes 301 instructions to 
fill it. Runtime has been longer, but only about 15%. I would not have expected a rise of 
50%, as there is overhead for starting the program.

What happens, if we activate `--trace`?

~~~
     Running `target/debug/lovas -r pgm/endless-stack.lva --print --stack-size 150 --trace`
Pgm { name: "pgm/endless-stack.lva", text: [2, 123, 32, 255, 251, 255] }
VM { stack: [], pc: 0, op_cnt: 0, trace: true, watermark: 0 }
Executing op 0x02
VM { stack: [123], pc: 2, op_cnt: 1, trace: true, watermark: 1 }
Executing op 0x20

[...]

Executing op 0x02
Runtime error!
Runtime=67.312973ms
op_cnt=301, pc=2, stack-depth=150, watermark=150
Error: StackOverflow
~~~

There is, of course, a lot of output, that I cut out. What is interesting is the change in execution time.
I ran this inside the CLion IDE by JetBrains. The console there will not be a very fast console, as it 
does a lot with that output coming through. But the impact of the logging is enormous! The runtime until 
we hit our stack overflow is more than 1000 times longer! The exact numbers don't mean anything; we are 
running unoptimised Rust code with debuginfo, and the bottleneck is the console. But it is still 
fascinating to see.
