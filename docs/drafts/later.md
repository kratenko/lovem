# Alles ist aus Stack gemacht

[...]

Adding a byte-oriented structure to the stack does however not fit very well with my current 
stack design of having i64 values. I can, of course, start slamming them on the 8 byte sized 
values, padding them to fit in. That would not even be too horrible, would not even need 
`unsafe` code. But it made me think.

The original idea: having every value 64 bit would make the design clear and simple. But 
experiencing how many values I push to the stack, it works against my main goal: light 
weight. That stack fills by 8 bytes for every value I put there, every variable I 
introduce. And if I would build that byte-chunk-inside-i64-thing I was musing about, 
I lose that simplicity anyway. I think, I at least want to try out working with a 
byte-sized stack. From my gut feeling, I would assume that the complexity added is 
not too bad, if you restrain yourself from becoming too fancy. And you will save a lot 
of ram in VM during execution (which translates to: the stack can be much smaller 
for an equivalent program).


# Bytes will be bytes

Yeah, it is the perfect time for ripping the stack apart and changing it to byte sized values;
the very last thing I did before going to sleep yesterday was adding unit tests for pushing 
and popping stack values. So hurray.

[...]