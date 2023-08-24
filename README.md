# cemb
## cemb is a small virtual machine that runs cASM(cemb Assembly). WARNING cemb project is still under development and still Turing Incomplete.

### What is CASM?
cASM Stands for cemb assembly. Its a small programming language that you work directly with stack for memory management
due to design of cemb.

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
