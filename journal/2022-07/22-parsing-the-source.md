---
entry: 22
published: 2022-07-27
tag: v0.0.8-journey
---

# Parsing the source

So far we have read an assembly source file into a string, and we got to know some new data 
structures. It is time we use the one to fill the other. Let us start parsing.

What we know so far is this:

~~~rust
/// Parse assembly source code and turn it into a runnable program (or create report).
pub fn assemble(name: &str, content: &str) -> Result<Pgm, AsmErrorReport> {
    let asm_pgm = AsmPgm::parse(name, content);
    asm_pgm.to_program()
}
~~~

## Assembler syntax
Our experimental assembler will begin using a simple syntax. Only one instruction per line, short 
opnames to identify the operation to be executed, optionally a single argument. I have written a 
short program:
[`hallo-stack.lass`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/pgm/hallo-stack.lass).

~~~
push_u8 123
push_u8 200
add
pop
fin
~~~

Straightforward. And you know the syntax already from my human friendly listings of bytecode.
Parsing that looks simple. We do want to allow adding whitespaces, though. 
And we want to allow comments, for sure. Our assembler needs to handle a bit of noise, as in 
[`noice.lass`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/pgm/noise.lass).

~~~
# This is an awesome program!
 push_u8 123
push_u8     200      # What are we using the # 200 for?


add
   pop


# let's end it here!
fin

~~~

Those two programs should be identical and produce the same bytecode.

## One line at a time
The `parse()` function we call creates an empty instance of `AsmPgm` 
and then processes the source file line after line, filling the 
`AsmPgm` on the way.

~~~rust
/// Parse an assembly program from source into `AsmPgm` struct.
fn parse(name: &str, content: &str) -> AsmPgm {
    // create a new, clean instance to fill during parsing:
    let mut p = AsmPgm {
        name: String::from(name),
        instructions: vec![],
        line_number: 0,
        text_pos: 0,
        error: None,
    };
    // read the source, one line at a time, adding instructions:
    for (n, line) in content.lines().enumerate() {
        p.line_number = n + 1;
        let line = AsmPgm::clean_line(line);
        if let Err(e) = p.parse_line(line) {
            // Store error in program and abort parsing:
            p.error = Some(e);
            break;
        }
    }
    p
}
~~~

`content.lines()` gives us an iterator that we can use to handle each line of the 
String `content` in a for loop. We extend the iterator by calling `enumerate()` on it; 
that gives us a different iterator, which counts the values returned by the first iterator, 
and adds the number to it. So `n` will hold the line number and `line` will hold the line's 
content.

We always keep track of where we are in the source. Because the `enumerate()` starts counting 
at `0` (as things should be), we need to add `1`. File lines start counting at `1`.
The first thing we do with the line is cleaning it. Then it gets processed by `parse_line(line)`.
If this produces an error, we will store that error and abort parsing. All our errors are fatal.
The final line `p` returns the `AsmPgm`. We do not use a `Result` this time, but the `AsmPgm` can 
contain an error. Only if its error field is `None`, the parsing was successful.

## Cleaning the noise
~~~rust
/// Removes all noise from an assembler program's line.
fn clean_line(line: &str) -> String {
    // Remove comments:
    let line = if let Some(pair) = line.split_once("#") {
        pair.0
    } else {
        &line
    };
    // Trim start and end:
    let line = line.trim();
    // Reduce all whitespaces to a single space (0x20):
    ANY_WHITESPACES.replace_all(line, " ").to_string()
}
~~~

We use multiple techniques to clean our input: splitting, trimming, regular expressions. When 
we are done, we only have lines as they look in 
[`hallo-stack.lass`](https://github.com/kratenko/lovem/blob/v0.0.8-journey/pgm/hallo-stack.lass). 
The cleaned line can also be completely empty. 

I want to add a word about that regexp in `ANY_WHITESPACES`. Where does it come from? 
I am using some more Rust magic there, and the crate `lazy_static`:

~~~rust
use lazy_static::lazy_static;
use regex::Regex;

// Regular expressions used by the assembler.
// lazy static takes care that they are compiled only once and then reused.
lazy_static! {
    static ref ANY_WHITESPACES: Regex = regex::Regex::new(r"\s+").unwrap();
    static ref OP_LINE_RE: Regex = regex::Regex::new(r"^(\S+)(?: (.+))?$").unwrap();
}
~~~

I do not pretend to understand the macro magic that happens here. But what happens, is that 
the regular expressions are compiled only once and then kept as some sort of global static 
immutable variable, that we can than use again and again all over the program as a reference.
Static references are a convenient thing in Rust, if you remember what I told you about 
ownership. You can always have as many references to immutable static variables, because there 
is nothing that can happen to them, and they exist throughout the complete runtime of the 
program.

## Parsing a clean line

~~~rust
/// Handles a single cleaned line from an Assembly program.
fn parse_line(&mut self, line: String) -> Result<(), AsmError> {
    if line == "" {
        // empty line (or comment only) - skip
        return Ok(());
    }
    if let Some(caps) = OP_LINE_RE.captures(&line) {
        let opname = caps.get(1).unwrap().as_str();
        let parm = caps.get(2).map(|m| m.as_str());
        return self.parse_instruction(opname, parm);
    }
    Err(AsmError::InvalidLine)
}
~~~

`parse_line()` processes each line. Empty ones are just skipped. We use another 
regular expression, to find out if they match our schema. Because we cleaned it 
the expression can be rather simple: `r"^(\S+)(?: (.+))?$"`. 
We look for one or more non-empty chars for our *opname*. It can be followed by 
a single argument, which must consist of one or more chars, separated by a single space.
That is our optional *oparg*. If the line fits, we found an introduction we can try to 
parse. That is the job of `parse_instruction()`. Everything that is neither empty nor 
an instruction, is an error, that we can simply return. It will abort the parsing and 
the caller will know, that there was an invalid line.

`parse_instruction()` can also run into an error. We use our tried pattern of returning 
a `Result` where the successful outcome does not carry any additional information
(which is why we return `Ok(())`). The error case will return an AsmError, that carries 
the reason for the error. And because of our the `Result` type and because of Rust's 
might enum system, we can simply return what `parse_instruction()` returns to us.

Handling the instruction itself will be handled in the next entry.
