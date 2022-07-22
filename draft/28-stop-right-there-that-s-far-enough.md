---
entry: 28
published: 2022-07-31
tag: v0.0.12-journey
---

# Stop right there, that's far enough!

__We introduce an optional execution limit to our VM.__

Since we have `goto`, we can write looping programs. With `if*` we have potentially looping 
programs as well. Both of this open the potential for endless loops. There are situations, in 
which endless loops are required. But often they are something to be avoided.

## Looping a long time
Let us look at a little program:
~~~ title="pgm/long-loop.lva" linenums="1"
# Looping a looooong time.
# This program will not run forever, but you will not see it terminate either.
  push_u8 0
loop:
  push_u8 1
  add
  dup
  ifgt loop
  pop
  fin
~~~

Someone messed up the loop condition there. If you run this program, it will be running for a long time. 
We start at zero and add to the value until our number is smaller than 0. Sounds impossible to reach 
for normal people, programmers will now better. Eventually we will reach the integer overflow, and our 
signed integer will loop around from its highest possible value to the lowest possible one. But do remember, 
what type we currently use to store our values: `i64`. So how big is that highest number?

    9223372036854775807

Is that a lot? That depends. Last entry I had my program loop for 1 million rounds. It took my modern 
laptop about half a second. So reaching that number should take 9223372036854.775807 times as long, that 
is around 4611686018427 seconds or just about 146135 years. Is that a lot?

Oh, and by the way, the Rust professionals reading this will have spotted a potentially false claim there.
While we run our program in debug mode, there will be no integer wraparound, instead the program will panic.
But that is besides the point.

## Limited execution
The reason I started writing *lovem*, is that I need an embeddable lightweight VM to execute programmable 
handlers when certain events occur on my restrained embedded devices. So we are talking about some form 
of user generated content that is executed as a program! We can never trust those programs to be solid.
We need a way to limit execution in some way, so that the device has the possibility to terminate those 
programs. There is an easy way to achieve that with what we already have. We put a limit on the number 
of operations the VM will execute.

We add a few lines to our VM's main loop:

~~~ rust title="src/vm.rs" linenums="137" hl_lines="9-12"
// Loop going through the whole program, one instruction at a time.
loop {
    // Log the vm's complete state, so we can follow what happens in console:
    if self.trace {
        println!("{:?}", self);
    }
    // Fetch next opcode from program (increases program counter):
    let opcode = self.fetch_u8(pgm)?;
    // Limit execution by number of instructions that will be executed:
    if self.instruction_limit != 0 && self.op_cnt >= self.instruction_limit {
        return Err(RuntimeError::InstructionLimitExceeded);
    }
    // We count the number of instructions we execute:
    self.op_cnt += 1;
    // If we are done, break loop and stop execution:
    if opcode == op::FIN {
        break;
    }
    // Execute the current instruction (with the opcode we loaded already):
    self.execute_op(pgm, opcode)?;
}
~~~

And of course we also add that new `RuntimeError::InstructionLimitExceeded` and a new 
field `pub instruction_limit: usize,` to our VM struct.

`lovas` gets a new optional parameter:

~~~rust title="src/bin/lovas.rs" linenums="34"
#[clap(long, default_value_t = 1000000, help = "Limit max number of instructions allowed for execution. 0 for unlimited.")]
instruction_limit: usize,
~~~

And we need to pass that to the VM in the `run()` function:

~~~rust title="src/bin/lovas.rs" linenums="38" hl_lines="6"
/// Executes a program in a freshly created lovem VM.
fn run(pgm: &Pgm, args: &Cli) -> Result<()> {
    // Create our VM instance.
    let mut vm = VM::new(args.stack_size);
    vm.trace = args.trace;
    vm.instruction_limit = args.instruction_limit;
    let start = Instant::now();
    let outcome = vm.run(&pgm.text);
    let duration = start.elapsed();
...
~~~

And, well, that's it. We now have an optional execution limitation that we default at 1 million.

## Testing it
~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/long-loop.lva --print
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas -r pgm/long-loop.lva --print`
Pgm { name: "pgm/long-loop.lva", text: [2, 0, 2, 1, 16, 3, 37, 255, 249, 1, 255] }
Runtime error!
Runtime=142.400812ms
op_cnt=1000000, pc=7, stack-depth=2, watermark=2
Error: InstructionLimitExceeded
~~~

We can adjust it easily:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/long-loop.lva --print --instruction-limit=100
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas -r pgm/long-loop.lva --print --instruction-limit=100`
Pgm { name: "pgm/long-loop.lva", text: [2, 0, 2, 1, 16, 3, 37, 255, 249, 1, 255] }
Runtime error!
Runtime=19.096µs
op_cnt=100, pc=7, stack-depth=2, watermark=2
Error: InstructionLimitExceeded
~~~

And we can just as well disable it completely:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/long-loop.lva --print --instruction-limit=0
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/lovas -r pgm/long-loop.lva --print --instruction-limit=0`
Pgm { name: "pgm/long-loop.lva", text: [2, 0, 2, 1, 16, 3, 37, 255, 249, 1, 255] }

~~~

Good luck waiting for this one. I hope you know how to terminate a running program on your system...
