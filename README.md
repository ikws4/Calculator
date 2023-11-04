# Calculator

A simple math expression evaluator written in rust.

# Usage

```bash
cargo run
```

# Functions

- abs
- ceil
- floor
- round
- sign
- sin
- cos
- tan
- asin
- acos
- atan
- ln
- log
- sqrt
- max
- min
- clamp
- clamp01

# Constants

- pi
- e

# Example

```bash
> 1 + 2 * (2 ^ 10) + ceil(10 / 3) + sin(2 * pi)
```

# Grammar

```
expression: addition
addition: multiplication ('+' | '-' multiplication)*
multiplication: unary ('*' | '/' | '%' | '^' unary)*
unary: '-'? parentheses
parentheses: '(' expression ')' | atom
atom: number | call
number: [0-9]+ ('.' [0-9]+)?
call: identifier ('(' arguments ')')?
identifier: [a-zA-Z][a-zA-Z0-9]*
arguments: expression (',' expression)*
```
