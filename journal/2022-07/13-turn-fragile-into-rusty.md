---
entry: 13
published: 2022-07-14
---

# Turn "fragile" into "rusty"

__After we got our Proof of Concept running, we clean up our code and make it look like a respectable 
Rust program.__

Did you play around with the program from the previous post? If you are new to Rust, you really 
should! At least mess around with our bytecode. You should find, that our VM does not react well to 
errors, yet. It simply panics! That is no behaviour for a respectable rust program.

We will make it more rusty, look at the enhanced version:

Repo:
https://github.com/kratenko/lovem/tree/v0.0.2-journey

main.rs:
https://github.com/kratenko/lovem/blob/v0.0.2-journey/src/main.rs

If you do not know your way around Rust, some of those things will be difficult to understand. It might be 
time to read up on some Rust, if you intend to follow my journey onwards. I will not explain everything here, 
but I will give you some leads right now, if you want to understand the things I did in that change.

## It is all in the enums
The most important thing to understand for you will be *Enums*. 
Yeah, I know. That is what I thought at first learning Rust. 
"I know enums. Yeah, they are handy and useful, but what could be so interesting about them?"

Well, in fact, enums in Rust completely change the way you are writing code. They are such an important 
part of the language that they have an impact on just about every part of it.

I introduced an enum to the code:

~~~rust
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    InvalidOperation(u8),
    StackUnderflow,
    StackOverflow,
}
~~~

It is obviously a datatype to communicate runtime errors of different nature. And I use it a bit 
like you would exceptions in some other languages.
Nevermind the `#[derive...]` part for now. That is just for fancy debug output (and a bit more).
Once you understand [line 33][line33]: `InvalidOperation(u8),`, you are on the right track!
To put it into easy terms: values of enums in Rust can hold additional values. And, as you see 
in our `RuntimeError`, not all values have to hold the same kind of additional value, or a value 
at all. This is, what makes enums really powerful.

If you know what happens in the return type of fn `push` in  [line 70][line70], you are golden.
The `Result` type can communicate a value on success or an error condition on failure. The great 
difference to typical exceptions form other languages is, that there is no special way to pass on 
the errors, as with exceptions that are thrown. It is just your normal `return` statement used.
And this is done, you guessed it, with enums. If you want to read up on `Result`, try 
understanding `Option` first. I am using that in my code, even though you cannot see it.

If you are wondering now about the return of fn `push`, that does not have a `return` statement 
to be seen, you should find out, while some of my lines do not have a semicolon `;` at the end, 
while most do. 

And then there is that tiny `?` in [line 101][line101].

Also find out what happens in the `match` in [line 166][line166]. It might help if you start 
with the `if let` statement.

Bonus points: [line 66][line66]. If that is clear to you, you need have no worries, you 
are into enums and how to use them

## Homework
So, this is what will get you through a lot here. Try to understand those in the given order:

  1. `Option`
  2. `Some(v)` vs. `None`
  3. `Result<v, e>`
  4. `Ok(v)` vs. `Err(e)`
  5. `if let Some(v) = `
  6. `match`
  7. `Result<(), e>`
  8. `Ok(())`
  9. `unwrap()`
  10. `?`
  11. Bonus: `ok()`, `ok_or()`, and their likes

If you understand for each of those, and why I put them in the list, you are prepared to handle most Rust 
things I will be doing in the next time. If you have problems with parts of it, still, move on. It 
gets better after a while, when you use them.

[line33]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L33
[line66]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L66
[line70]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L70
[line101]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L101
[line161]: https://github.com/kratenko/lovem/blob/9d97281bd6ffdae894f8052c91ea32d1d761fdb2/src/main.rs#L161
