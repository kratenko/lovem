---
entry: 31
published: 2022-07-31
tag: v0.0.15-journey
---

# Frames on the stack

__We take a look at a frame stack solution that works on the main stack.__

In my last entry, I said I was not happy with introducing a second stack to the VM. Today we prove that 
doing this with one stack only is possible. It is not the solution I am going to use, but it explores 
a new approach, that explicitly separates parameters from values that should not be accessible from the 
called function.

## `call` with parameters
We modify our `call` opcode slightly. There is no change in the code, only in the comments documenting, how the 
stack will be effected.

~~~ rust title="src/op.rs" linenums="117" hl_lines="3"
/// opcode: Save return position and jump. Pass n values to called function, where n is popped.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const CALL: u8 = 0x27;
~~~

Our assembler does not need any change, as the opnames and their opargs don't change.

The VM has the most changes. For one, the `fstack` gets nicked, as we do not want to use it.
And then there is the change to the handlers for `call` and `ret`, of course:

~~~ rust title="src/vm.rs" linenums="325"
op::CALL => {
    let d = self.fetch_i16(pgm)?;
    let n = self.pop()? as usize;
    if self.stack.len() < self.fb + n {
        // there are not enough values on the stack to pass to the function called
        return Err(RuntimeError::StackUnderflow);
    }
    // push frame to stack
    self.push(n as i64)?;
    self.push(self.pc as i64)?;
    self.push(self.fb as i64)?;
    // move function parameters to the top, move the frame base down:
    let end = self.stack.len();
    let fstart = end - 3;
    self.stack.moveslice(fstart..end, fstart-n);
    // move frame base, so that frame starts at first parameter:
    self.fb = self.stack.len() - n;
    // jump into function:
    self.relative_jump(pgm, d)
},
op::RET => {
    let n = *self.stack.get(self.fb - 3).unwrap() as usize;
    if self.stack.len() != self.fb + n {
        return Err(RuntimeError::InvalidReturn);
    }
    // move frame to stack top
    self.stack.moveslice(self.fb-3..self.fb, self.stack.len() - 3);
    // pop frame
    self.fb = self.pop()? as usize;
    self.pc = self.pop()? as usize;
    self.pop()?;
    Ok(())
},
~~~

That looks way too complicated! And maybe it is. I stopped cleaning it up and making it easier to understand, 
because I don't think it will end in the VM at all. But what does it do?

## Calling convention
The calling convention for this is not too complicated in its core. The idea is, that you push all values 
that you want to pass to the stack, followed by the number of values pushed. Then you call the function. 
When it returns, the number of values (the last thing you pushed) will be gone, but there will still be as 
many values as you pushed as parameters. Those are used as in parameters as well as out parameters (return values, 
if you want). If the number for in and out parameters differs, you will simply have to ignore those, that you 
don't need. This is both simple and flexible.

## Stack manipulation for calls
We still need to push our frame information on that stack as well. This gets a little messy, as you can 
witness in the code above. For the call, we are moving the frame information below the parameters, 
after those have been pushed. What happens? Let us take a look. The following is an attempt to illustrate 
the contents of the stack during function call and return. There are 5 states of the stack during execution
listed from left to right. Values are pushed to the top.

~~~
Legend:
  pc: Program Counter (of previous frame, stored in stack)
  fb: Frame Base (of previous frame, stored in stack)
  n: number of parameters passed
  p0, p1, p2: parameters passed
  v0, v1: values that are on the stack before we start the call (inside frame of calling function)
  ->: current position of fb pointing to

Stack change:

                      
                 fb     p2
                 pc     p1
         n       n    ->p0
         p2      p2     fb     p2
         p1      p1     pc     p1
         p0      p0     n      p0    
  v1     v1      v1     v1     v1
->v0   ->v0    ->v0     v0   ->v0
  |      |       |      |      |
  |      |       |      |      +-- after return
  |      |       |      |
  |      |       |      +-- frame moved down, fb changed (inside function)
  |      |       |
  |      |       +-- frame pushed, not before move of frame
  |      |
  |      +-- parameters pushed
  |
  +-- before we start
~~~

Not only does this pass parameters and handles the return, it also protects values of lower frames from 
being manipulated, by moving the FB.

## Run it
Let's try it out with a modified version of our `call.lva`.

~~~ title="pgm/call.lva" hl_lines="3 5"
start:
    push_u8 5
    push_u8 1
    call square
    push_u8 1
    call square
    out
    fin

square:
    dup
    mul
    ret
~~~

We had to add two more `push_u8` instructions, that indicate the number of parameters passed.
Does it still work?

~~~
     Running `target/debug/lovas -r pgm/call.lva --print`
Pgm { name: "pgm/call.lva", text: [2, 5, 2, 1, 39, 0, 7, 2, 1, 39, 0, 2, 6, 255, 3, 18, 40], vars: 0 }
Out: 625 (@12)
Terminated.
Runtime=18.741µs
op_cnt=13, pc=14, stack-depth=0, watermark=5
~~~

Compared with our previous version, the program text (=bytecode) is 4 bytes longer, because of the 
two `push_u8`. Execution uses two more instructions (again, the two `push_u8`). And the watermark 
is 5 instead of 2 - that are the 3 values of the frame on the stack. But we do not need the second 
stack any longer, which was not shown in the output of the previous version. It feels a bit excessive 
using all those bytes. But that would feel much less so, if we had real functions, that do stuff
(because they would be longer in themselves).

The runtime got longer by a factor of about 2.5 - that is no real surprise, because that move function is 
complicated compared to the rest of what we are doing.

## Conclusion
I am not too happy with the result. But it gave me a few ideas, when I was trying to make it better. 
We will work on that during the coming journal entries. The assembler will get more complicated. I must 
watch out that I do not turn it into a higher language by accident. It should remain assembly, but there 
is no need to support features that will only be there to break things. I'm looking forward to seeing, how 
far I will take it.

If you could not follow what the code above is doing, do not worry too much. I hope the solution I'm 
heading for will be simpler. At least inside the VM. I'm not that afraid to make the assembler more 
complex.
