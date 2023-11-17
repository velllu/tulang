# Tulang
A simple "programming language" (barely so) that compiles to turing machines, this was made for a school project and it is *not* finished.

# Instructions
## The two types of functions
### Normal functions
Every normal function has a "direction" and "replacements" arguments, here's an example:
```
move_to_char, right, x -> y
```
this function is explained later, just know that only normal functions have these kind of arguments.

### Compile time functions
These are functions that are basically 0 cost, they don't compile to any actual lines of turing machine code, but they are needed to make other functions work.
They do not have "direction" and "replacements" arguments because of this fact.

## Alphabet instruction (compile time)
This should be the first instruction of every tulang project, it detones the alphabet, basically the possible characters that the turing machine can contain.
This is needed to know what characters to work with in the `move_to_char` function for example.  
Here is how to define an alphabet composed of the characters "xyz"
```
alphabet, xyz
```

## MoveToChar instruction
Example, this makes it so it's gonna move to the right until it hits a blank character (`-` means a blank character),
and while it travels the characters, it's gonna replace As with Bs.
```
move_to_char, right, -, a -> b
```
You can switch "right" with "left", not that this language does *not* support the machine being still.

## BeginLoop instruction (compile time)
This marks the start of a loop, not that this language only supports one loop at a time, you cannot have nested loops.  
Usage:
```
begin_loop
```

## EndLoop instruction
This exits the loop started with `begin_loop`, here's how it looks like:
```
end_loop, right, x -> y
```
Note that this is *not* a compile time instruction and as such supports replacements and direction
