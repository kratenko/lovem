---
entry: 26
published: 2022-08-11
tag: v0.0.10-journey
---

# You labeled me, I'll label you

__We add a feature to our assembler that we overlooked before.__

Over the last few entries we created ourselves a really useful little assembler program. I hope you 
played around with it and enjoyed not having to write bytecode directly. If you did, you should have 
noticed that I left out a really important detail. Remember when I was complaining about how bad 
writing bytecode is? And that it got even worth, when we introduced jumps? Yeah, I did not solve 
that problem at all. If anything, I made it worse, because you still have to count the relative 
bytes to your destination, but you do not see those bytes any longer. You just have to know, how 
many bytes each instruction will produce.

## Labels
There was so much already going on in that assembler program, that I did not want to introduce more 
complexity up front. Let's fix that now: we will introduce a way to give a position inside your 
program a name, so that you can `goto` that name later. And in good tradition, we will call this 
names *labels*.

The traditional way of defining labels in assembly is by writing them first thing on a line, followed 
by a colon `:`. Take a look at this little program, 
[label.lva](https://github.com/kratenko/lovem/blob/v0.0.10-journey/pgm/label.lva). It is neither good 
style, nor does it do anything useful, but it shows us labels:

~~~linenums="1" hl_lines="5 9" title="pgm/label.lva"
# A small demonstration of how labels work with goto.
push_u8 1
goto coda

back:
  push_u8 3
  fin

 coda:  push_u8 2
goto back
~~~

There are two labels defined here: `back` in line 5, and `coda` in line 9. A label definition is a short string 
that is directly followed by a colon `:`. We restrict it to letters, numbers, and underscore, with a letter at the 
front. For the curious, the regex is: `^[A-Za-z][0-9A-Za-z_]{0,31}$`. As you can see in the example, there can 
be an optional instruction in the same line as the label definition. Now, how will our assembler parse those?

## Reconstruction
First of all, I did a little reconstruction inside `asm.rs`, because I did not like how the parsing was 
done inside an associated function, that also created the `AsmPgm` instance. That seems messed up. After the 
change, the `fn assemble()` creates the instance itself and then calls a method on it, to parse the 
source code. Here is the new version:

~~~rust linenums="367" title="src/asm.rs"
/// Parse assembly source code and turn it into a runnable program (or create report).
pub fn assemble(name: &str, content: &str) -> Result<Pgm, AsmErrorReport> {
    // create a new, clean instance to fill during parsing:
    let mut asm_pgm = AsmPgm {
        name: String::from(name),
        instructions: vec![],
        line_number: 0,
        text_pos: 0,
        error: None,
        labels: Default::default(),
    };
    // evaluate the source code:
    asm_pgm.process_assembly(content);
    // convert to Pgm instance if successful, or to Error Report, if assembly failed:
    asm_pgm.to_program()
}
~~~

And there is no problem with us changing the code like this. The only public function inside 
`asm.rs` is that `pub fn assemble()`. All methods of `AsmPgm` are private and therefore 
internal detail. Not that it would matter at this state of development, but it demonstrates 
how separation of public API and internal implementation work.

What is also new in that function is a new field inside `AsmPgm`: `labels`.

~~~rust hl_lines="6"
/// A assembler program during parsing/assembling.
#[derive(Debug)]
struct AsmPgm {
    ...
    /// A map storing label definitions by name with there position in bytecode.
    labels: HashMap<String, usize>,
}
~~~

It is a HashMap (aka. associative array in other languages). This is where we put all 
label definitions we find, while parsing the source file. It maps the label's name to 
its position inside the bytecode. Here we can look up where to jump, for a goto that 
wants to jump to a label.

This is what our parsing methods now look like:

~~~rust linenums="327" hl_lines="4 5"
fn process(&mut self, content: &str) -> Result<(), AsmError> {
    // Go over complete source, extracting instructions. Some will have their opargs
    // left empty (with placeholders).
    self.parse(content)?;
    self.update_instructions()
}

/// Process assembly source code. Must be used with "empty" AsmPgm.
fn process_assembly(&mut self, content: &str) {
    // this function is just a wrapper around `process()`, so that I can use the
    // return magic and don't need to write the error check twice.
    if let Err(e) = self.process(content) {
        self.error = Some(e);
    }
}
~~~

The important part is, that we have to steps now. We parse the complete source, as before. 
The second run is needed to write the actual relative jump address to the instructions. We do 
not know them during parsing, at least not for jumps forward.

## Parsing label definitions
I got a little fancy again, while writing the function for parsing label definitions:

~~~rust linenums="257" hl_lines="13"
/// Parses and extracts optional label definition from line.
///
/// Looks for a colon ':'. If one exists, the part before the first colon will be
/// seen as the name for a label, that is defined on this line. Instructions inside
/// the program that execute jumps can refer to these labels as a destination.
/// Lines containing a label definition may also contain an instruction and/or a comment.
/// This can return `AsmError::InvalidLabel` if the part before the colon is not a valid
/// label name, or `AsmError::DuplicateLabel` if a label name is reused.
/// If a label could be parsed, it will be stored to the `AsmPgm`.
/// On success, the line without the label definition is returned, so that it can be
/// used to extract an instruction. This will be the complete line, if there was no
/// label definition.
fn parse_label_definition<'a>(&mut self, line: &'a str) -> Result<&'a str, AsmError> {
    if let Some((label, rest)) = line.split_once(":") {
        let label = label.trim_start();
        if VALID_LABEL.is_match(label) {
            if self.labels.contains_key(label) {
                Err(AsmError::DuplicateLabel(String::from(label)))
            } else {
                self.labels.insert(String::from(label), self.text_pos);
                Ok(rest)
            }
        } else {
            Err(AsmError::InvalidLabel(String::from(label)))
        }
    } else {
        Ok(line)
    }
}
~~~

The method is trying to find a label definition in the line, and if so, handles it. We use our 
trusted `Result<>` returning, to communicate potential errors. But instead of `Ok(())`, which 
is the empty okay value, we return a `&str` on success. This is because there might also be 
an instruction in the line. If we find a label definition, it returns the line after the colon.
If there is none, it returns the complete line it got. This gives us the lines as we used to 
get before we introduced labels. Great. But what is that weird `'a` that shows up in that 
highlighted line everywhere?

## Lifetime
Yeah, this is where it becomes rusty, again. I said, in an early post, that you would hate the 
Rust compiler and its pedantic error messages. The thing Rust is most pedantic about, is ownership 
and access to values you do not own. We are working with references to `String`s here. A `&str` 
references the bytes inside that `String` directly (a `&str` need not reference a `String`, but it 
does here). We did that before, where is the problem now? This is the first time we are 
returning a `&str`.

When you are using references, Rust makes sure that the value you are referencing exists at least 
as long as the reference exists. That is easy for functions, as long as you drop every reference 
you have when you are done. But in this function, we return a reference to the parameter we 
got. Rust cannot allow that without some special care. When I remove the `'a` parts of the method, 
I get a compilation error:

~~~
error[E0623]: lifetime mismatch
   --> src/asm.rs:277:21
    |
269 |     fn parse_label_definition(&mut self, line: &str) -> Result<&str, AsmError> {
    |                                                ----     ----------------------
    |                                                |
    |                                                this parameter and the return type are declared with different lifetimes...
...
277 |                     Ok(rest)
    |                     ^^^^^^^^ ...but data from `line` is returned here
    |
    = note: each elided lifetime in input position becomes a distinct lifetime
help: consider introducing a named lifetime parameter and update trait if needed
    |
269 |     fn parse_label_definition<'a>(&'a mut self, line: &'a str) -> Result<&str, AsmError> {
    |                              ++++  ++                  ++
~~~

The compiler tells me, that I messed up the lifetimes. It even proposes a change that introduces lifetime 
parameters (but gets it slightly wrong). What do we do with the `'a`?

Well we introduce a lifetime parameter called `a`. The syntax for that is the apostrophe, which looked weird 
to me at start, but it is so lightweight, that I came to like it. It is custom, to just call your lifetimes 
`'a`, `'b`, ... &ndash; they normally don't have a long scope anyway. The thing we are telling the compiler 
with this parameter is this: the lifetime of the returned `&str` is dependent on the lifetime of the 
parameter `line: &str`. So whenever the reference the function is called with runs out of scope, the 
reference that was returned must be out of scope as well.

### An example
This is a concept that is new to many programmers when they learn Rust. I think, what we do here demonstrates 
it quiet well. Let us look at what happens for line 9 of our assembly program:

~~~ title="pgm/label.lva" linenums="9"
 coda:  push_u8 2
~~~

Our function receives a reference to a `String` holding that line: `" coda:  push_u8 2"`. It finds the 
label `coda` and stores it inside `self.labels`. Its work is done, but there might be more to this line.
It returns a reference to a substring of it (`&str` are actually slices; they can reference only a 
part of a `String`'s data). That is what we return, a reference to the part data inside the `String`, starting 
at the first char after the colon, so it looks like this `"  push_u8 2"`. It is *not* a copy, it is the same 
area inside the computer's memory! So if you want to make certain, that there are no accesses to memory 
after its content has run out of scope (use after free, or use of local variable after it runs our of scope), 
you *must not* allow access to it, unless you are sure the value still exists. And this is what Rust does. This 
is what makes Rust a secure language. *Many* bugs and exploits in the world exist, because most languages do 
not check this, but leave the responsibility to the programmer. And the really cool thing about Rust is, 
it does this completely at compile time, as you can see by the fact that we got a compiler error.

The way we call our function is not a problem at all:

~~~rust title="src/asm.rs" linenums="292"
for (n, line) in content.lines().enumerate() {
    // File lines start counting at 1:
    self.line_number = n + 1;
    let line = self.parse_label_definition(line)?;
    let line = AsmPgm::clean_line(line);
    self.parse_clean_line(line)?;
}
~~~

Our initial `line` comes from line 228. It is already a reference, because `content.lines()` 
is also giving us a reference to the memory inside of `content`. That is a reference already, 
the `String` variable that holds (and owns) the data lives inside `lovas.rs`:

~~~ rust title="src/bin/lovas.rs" linenums="67" hl_lines="2 7"
    // read complete source file into String:
    let content = std::fs::read_to_string(&args.source)
        .with_context(
            || format!("could not read file `{}`", &name)
        )?;
    // run the assembler:
    match asm::assemble(&name, &content) {
~~~

We do not copy any of that bytes along the way. The first time we do that is in `clean_line()`. 
Returning a `&str` will not work there, because we actually modify the contents of the string, by 
replacing characters inside it.
Have you ever tried to work with inplace "substrings"
(I mean char arrays, like this `char *str`), without modifying the contents (placing `\0` bytes). It 
is not fun. In Rust, it can be, if you understand lifetime restrictions.

### Easy way out
If you run into problems with your `&str` inside a Rust program, there is often an easy way to get 
around that. You can simply create a new `String` from your `&str`, as we do in `clean_line()`. That 
will copy the bytes. For our program, that would have been no problem at all. Cloning a few bytes of 
source code for every line during assembly would cost us next to nothing. You would not notice in execution time.
But things are different when you need to quickly handle long substrings in a program. Think of a 
diagnostic job on a busy server. And remember that `String`s will be created on the heap. That is a 
complexity that you sometimes want to avoid. When programming microcontrollers, there is a chance that 
you do not even have a memory allocator at your disposal. And microcontrollers is, what we are aiming for 
in our project. There are already some parts of lovem, that we will need to change, because of that. 
But that is a story for another time. I just thought that this was a nice little example to introduce 
you to lifetime parameters. We will need them at some point...

## Run it already!
This is a long entry already. You can look at the complete state of the assembler directly in the 
sourcecode. You should know how to find the tags inside the repo by now. But I want to execute our 
new program, using the labels, before I end this. Here it is again:

~~~ title="pgm/label.lva" linenums="1"
# A small demonstration of how labels work with goto.
push_u8 1
goto coda

back:
  push_u8 3
  fin

 coda:  push_u8 2
goto back
~~~

We need to execute it with the `--trace` flag, or we will not see anything:

~~~
kratenko@jotun:~/git/lovem$ cargo run --bin lovas -- -r pgm/label.lva --print --trace
   Compiling lovem v0.0.10 (/home/kratenko/git/lovem)
    Finished dev [unoptimized + debuginfo] target(s) in 1.33s
     Running `target/debug/lovas -r pgm/label.lva --print --trace`
Pgm { name: "pgm/label.lva", text: [2, 1, 32, 0, 3, 2, 3, 255, 2, 2, 32, 255, 248] }
VM { stack: [], pc: 0, op_cnt: 0, trace: true, watermark: 0 }
Executing op 0x02
VM { stack: [1], pc: 2, op_cnt: 1, trace: true, watermark: 1 }
Executing op 0x20
  Jump from 5 by 3
VM { stack: [1], pc: 8, op_cnt: 2, trace: true, watermark: 1 }
Executing op 0x02
VM { stack: [1, 2], pc: 10, op_cnt: 3, trace: true, watermark: 2 }
Executing op 0x20
  Jump from 13 by -8
VM { stack: [1, 2], pc: 5, op_cnt: 4, trace: true, watermark: 2 }
Executing op 0x02
VM { stack: [1, 2, 3], pc: 7, op_cnt: 5, trace: true, watermark: 3 }
Terminated!
VM { stack: [1, 2, 3], pc: 8, op_cnt: 6, trace: true, watermark: 3 }
Terminated.
Runtime=65.598µs
op_cnt=6, pc=8, stack-depth=3, watermark=3
~~~

The program has three `push_u8` operations. If you executed them in the order of the source code, 
they would push `[1, 3, 2]` to the stack. But because of the `goto` instructions, they are not 
executed in that order. You can see the jumps in the trace, and you can see that the stack at 
termination holds the values in this order: `[1, 2, 3]`.

Not much of a program, but it shows you, how our new labels work. And finally: no more counting bytes!

## Homework
Our programs `endless.lva` and `endless-stack.lva` no longer work, because we changed how the 
`goto` instruction must be written. Can you fix them?
