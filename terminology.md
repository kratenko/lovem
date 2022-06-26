For documentation, conversations, and source code of Lovem to be 
precise, terms must be well-defined and use correctly. This is the 
place where I try to do that. 

Since the project is young and changing, terms might still change, 
if I am not happy with them. This file should act as the single 
source of truth. There will be missuses inside code, documentation, 
and journal.

There are some terms that should be used very precisely and not 
mixed up. Examples are `operation` vs. `instruction`, or 
`arguement` vs. `parameter`.

# Definitions
push
: Put a new value on top of the stack, increasing its size by one.

pop
: Remove the value on top of the stack, decreasing its size by one.

load
: Take a value from somewhere and push it on the stack

store
: Pop a value from the stack and put it somewhere (e.g. a variable)

operation
: An action that can be performed by the ALU, like ADD or STORE.
: Each operation is identified by its unique opcode. Some operations need
: an argument (oparg) for the action they are perfomring.

opcode
: A number 0-255 identifying a specific operation.
: As part of the instruction they are written in the bytecode.

oparg
: The argument given to an operation by an instruction.
: This can be zero, one, or multiple bytes long, depending on the operation, 
: but the number of bytes is fixed for each operation.
: Opargs encode an immediate values or an address for the operation.
: As part of the instruction they are written in the bytecode.
: Rational: The term is borrowed from Python (to settle the "parameter" vs "argument"
: debate). Java seems to call them "operands" (according to wikipedia), which I 
: find misleading, as I think of the values popped from the stack as operands.

instruction
: An entity defining a single action to be executed by the VM.
: This consists of an opcode, defining which operation to execute, and an
: argument (oparg), providing the operation with needed information.
: Multiple instructions produce a program. Instructions are encoded in 
: bytes (each instruction taking at least one byte, possibly more, 
: currently up to 9). Encoded in bytes, instructions are the text part of
: the bytecode.

ALU (not sure about that term, yet)
: Arithmetic logic unit, the entity that executes operations.
: In our case of a VM, it is, of course, virtual itself and just a bunch of code.

optoken:
: (assembly) Argument in an assembler program instruction as a string identifier.
: It is a named identifier for easier programming, indication jump destinations 
: or variables' names. They will be replaced in bytecode by numeric addresses 
: or identifiers.
