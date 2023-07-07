Frame
  - pc
  - fb

push return


Line types:

~~~
# Variable definitions:
var name
var v1, v2, vn

# Function definiton:
foo():
foo_with_parms(p1, p2, p3):

# Label definition
label_definition:

# Instruction
opcode
opcode oparg
~~~

~~~
  g0     g0
  g1     g1
->l0     l0
  v0     v0
  v1     v1
  p0     n
  p1     pc
  n      fb
       ->p0
         p1
         
~~~