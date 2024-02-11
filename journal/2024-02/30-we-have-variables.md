---
entry: 30
published: 2024-02-11
tag: v0.0.13-journey
---

# We have variables!

__A stack alone is not that mighty. But now we can stow data away.__

I implemented variables for the VM. And I did it in a way, that will freak out 
programmers, who have only ever worked with high languages in well-behaved 
environments &ndash; we now have variable support, but for global variables only.

Why would I do that? Well, it was easy. You might be surprised how easy it was. 
And it helps a lot in having something useful. For what I am going for, it would 
actually be a viable thing, too. You could do a lot. But don't worry, I want local 
variables, too.

## Tell me were to stick it
Variables need to live somewhere. When I first talked about stack machines, I 
said that "no other *direct* manipulations of the stack [were] allowed [but push or pop]." 
We will now see, why I added the *direct* there.

Variables hold values; words, to be more precise. We have an entity, that can hold an 
arbitrary number of words: the stack. So, what is the idea? When I write a program, 
I will know how many variables it will need. Actually, our assembler now can do that for us.
When I pass the program to the VM for execution, it looks at that number, and pushes that 
many zeros on the stack. Then it marks the current stack position as the new bottom. 
It does that by the newly introduces special *Frame Base Register (FB)*.

What's with that funny name? This is something I will need later, when I introduce real 
function calls inside the VM. A call will create a new frame that is somewhat like a 
new local execution environment. This will also allow for local variables (told ya, I want those).
But for now we have up to 256 global variables at our disposal. That is quite a bit. 

## Variable operations
There are two new operations for handling global variables:

  * `store`: pop a value from the stack and store it in the global variable identified by the 1-byte oparg.
  * `load`: read value from the global variable identified by the 1-byte oparg and push it to the stack.


## Variables in the assembler
This took more work than the changes in the VM. That is good, because we want to hide complexity away from 
the VM. The assembler runs on a powerful computer, and typically programs are run more often than they are 
assembled/compiled. I want named variables in assembler source. The VM works only with numbers to identify 
them. Our assembler translates that for us.

`store` and `load` each take the name of a variable as argument. When the assembler finds a new variable name, 
it is assigned a number (starting at 0). We actually just chunk them in a Vector and run through it everytime. 
We only support 256 variables, so there is no need to optimise there. It's fast enough. The index number is 
written as `u8` as a single byte oparg. I leave it to you to look at the new source code in `asm.rs` this time.
It is not too hard, and you should know enough Rust by now.

## A new Program
There is more information to store for a Program now, than only the text (aka. the bytecode): the global 
variables. The information we store is just the number of variables the program has. That is all we need, 
we are not interested in their names. And it is the bytecode's responsibility, to access the correct variables.

But since we now need that information in the VM, we finally change the parameter passed to `run()` from 
`&[u8]` to `&Pgm`. That is what caused the most changes inside `vm.rs`. The real additions are few.

## Variables in the VM
The VM itself gets a new field: `fb: usize`. That is the frame base register, and it currently does nothing 
but point to the position inside the stack behind the last global variable. So with zero variables, nothing 
changes. We also add `RuntimeError::InvalidVariable`.

Initialising the VM now includes making space for the variables:
~~~rust title="src/vm.rs" linenums="146"
// create global variables in stack:
for _ in 0..pgm.vars {
    self.push(0)?;
}
self.fb = pgm.vars as usize;
~~~

Popping values now needs to respect the frame base register, so it now looks this:
~~~rust title="src/vm.rs" linenums="68"
/// Tries and pops a value from value stack, respecting frame base.
fn pop(&mut self) -> Result<i64, RuntimeError> {
    if self.stack.len() > self.fb {
        Ok(self.stack.pop().unwrap())
    } else {
        Err(RuntimeError::StackUnderflow)
    }
}
~~~

And we need operation handlers, of course:
~~~rust title="src/vm.rs" linenums="304"
op::STORE => {
    let idx = self.fetch_u8(pgm)?;
    if idx >= pgm.vars {
        Err(RuntimeError::InvalidVariable)
    } else {
        let v = self.pop()?;
        self.stack[idx as usize] = v;
        Ok(())
    }
},
op::LOAD => {
    let idx = self.fetch_u8(pgm)?;
    if idx >= pgm.vars {
        Err(RuntimeError::InvalidVariable)
    } else {
        self.push(self.stack[idx as usize])?;
        Ok(())
    }
},
~~~

That's it. We now support variables!

## Show me your values
I added another operation with the opname `out`. It pops a value from the stack and prints it 
to stdout. This is not an operation that you would normally want in your VM. Output should be 
generated by function calls. But we don't have those, yet. I want something to easily show values 
during development, so you can see what happens, without always using `--trace`. We can always 
remove it, later. There is nothing new to that operation, so I won't discuss the code here.

## A new program!
~~~title="pgm/duplicate.lva" linenums="1"
# A program demonstrating use of variables.
start:
    # val = 1
    push_u8 1
    store val   # variable is declared implicitly here. We only have one type: i64
    # for loop, 5 rounds:
    push_u8 5
loop:
    # val = val * 2:
    load val
    push_u8 2
    mul
    store val
    # check loop counter:
    push_u8 1
    sub
    dup
    ifgt loop
end:
    pop
    # output final value of val
    load val
    out
    fin
~~~

The program is documented with comments. And you might have noticed that I define labels that I never use.
I just want to structure the program and name its parts. We don't have functions, so I use what we have.

~~~hl_lines="6"
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/duplicate.lva --print
   Compiling lovem v0.0.13 (/home/kratenko/git/lovem)
    Finished dev [unoptimized + debuginfo] target(s) in 2.66s
     Running `target/debug/lovas -r pgm/duplicate.lva --print`
Pgm { name: "pgm/duplicate.lva", text: [2, 1, 4, 0, 2, 5, 5, 0, 2, 2, 18, 4, 0, 2, 1, 17, 3, 37, 255, 242, 1, 5, 0, 6, 255], vars: 1 }
Out: 32 (@46)
Terminated.
Runtime=18.156µs
op_cnt=47, pc=25, stack-depth=1, watermark=4
~~~

It outputs a 32. That is good, because we start with a 1 and multiply it by 2 five times. We can write 
programs!

## Oh... and a bugfix
I found out that I introduced a bug when writing the parsing label definitions. 
I parsed for the colon `:`, before I removed the comments. So a line with no 
label definition, but with a comment containing a colon did produce a parsing error.

~~~ hl_lines="4"
# this was fine
:label # this was fine
:another # even this : was fine
# but this would produce an error: just a colon in a comment
~~~

I fixed that by removing comments from lines first.
