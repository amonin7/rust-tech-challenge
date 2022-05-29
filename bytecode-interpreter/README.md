# Bytecode interpreter

## Installation guide

```shell
git clone git@github.com:amonin7/rust-tech-challenge.git
cd rust-tech-challenge/bytecode-interpreter
cargo run
```

After that, you can just write your ByteCode and you will get the result. The example:
```text
LOAD_VAL 1
WRITE_VAR ‘x’
LOAD_VAL 2
WRITE_VAR ‘y’
LOOP 5
READ_VAR ‘x’
LOAD_VAL 1
ADD
WRITE_VAR ‘x’
END_LOOP
READ_VAR ‘x’
READ_VAR ‘y’
MULTIPLY
RETURN_VALUE
```

You should see in your console
```text
Result is 12
```

The example above represents the next pseudocode:
```javascript
function f() {
    x = 1
    y = 2
    loop for 5 iterations {
        x = x + 1
    }
    return x * y
}
```

**The list of possible operations:**
```text
LOAD_VAL(INTEGER)
LOOP(INTEGER)
READ_VAR(STRING)
WRITE_VAR(STRING)
ADD
SUBTRACT
MULTIPLY
DIVIDE
END_LOOP
RETURN_VALUE
```

## Task

### Main task

You are a TA at a university, and you want to evaluate your student’s homework without executing their (untrusted) code.
You decide to write a small web-service that takes bytecode as input, and interprets the results.
The bytecode language you need to support includes basic arithmetic and variables. 
The bytecode language is stack, rather than register based. 
ByteCode (right) is given for the following pseudocode (left):
```javascript
function f() {
    x = 1               // LOAD_VAL 1
                        // WRITE_VAR ‘x’
    
    y = 2               // LOAD_VAL 2
                        // WRITE_VAR ‘y’
    
    return (x + 1) * y  // READ_VAR ‘x’
                        // LOAD_VAL 1
                        // ADD
                        // READ_VAR ‘y’
                        // MULTIPLY
                        // RETURN_VALUE
}
```
Add a data type `ByteCode` that can represent bytecode like in the example above, along with an interpreter for said bytecode.

### Extension

Extend your interpreter with loops. In particular:
1. Extend your `ByteCode` data type with suitable instructions to support loops
2. Modify your interpreter to support said instructions
3. Try it out and see if it works :)