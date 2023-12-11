# cemb
## cemb is a small virtual machine that runs cASM(cemb Assembly). WARNING cemb project is still under development.

### What is CASM?
cASM Stands for cemb assembly. Its a small programming/scripting language that you work directly with stack for memory management
due to design of cemb. Each file is their own function.

## Examples:
### WARNING Documentation is still WIP.
```
printline 'cemb is awesome!'
```
This example prints "cemb is awesome!".
```
printline cemb.stack
```
This example prints entire cemb variable stack.
```
printline cemb.fromstack 0
```
### Variables
cASM uses a 0 based indexing like many programming languages.
This example prints element 0 (which is first) from Memory stack.
```
let str :: String = 'Hello World'
```
Let break this statement down:
- ```let```<- This is variable declaration keyword. In cemb every variable is immutable unless they are removed from Memory Stack.
- ```str``` <- This is Variable name.
- ```::``` <- Type Indicator.
- ```String``` <- This is the Type. 
- ```=``` <- -_-.
- ```'Hello World'``` <- This the Value.

in cASM once a variable is declared, it will be immediately pushed to stack.
By default every variable is immutable.
There is 4 Possible types in cASM:
1. String: A String is an array of characters.
2. Float: A 64-Bit floating point value.
3. Int: A 64-Bit whole value integer.
4. Char: A singular character.

Lets look at examples that uses each type:
```
let string :: String = 'The Answer to everything'
let float :: Float = 4.2
let integer :: Int = 42
let char :: Char = 'C'
```
#### Memory Management
In cASM, variables as we discussed previously are immediately pushed to the stack once they are declare. 
There is no auto-memory management in cASM. 
We have two main methods for memory management in cASM but we need to understand the basics of the variable references in cemb.
In cASM the variable name does not mean much. The most of the variable operations in cASM are done with the address of the variable in the stack.
Examples to make you understand it better.
```
let integer :: Int = 42
printline cemb.stack 0
// There is one variable in the stack. The Indexing is 0 based in cASM
```
Now we can learn the methods for memory management:
- ```dealloc_full_stack```
This resets the whole stack to 0. No variables will remain after this
- ```dealloc_certain_element 0```
This deallocates the variable that is in index 0 at stack.


