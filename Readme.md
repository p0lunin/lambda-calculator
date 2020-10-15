## Lambda-calculator
Lambda-calculator is a simple command line application to work with lambda
calculus.

### How to use
For install you need a cargo, rustc and git. Then run following commands:
```shell script
git clone https://github.com/p0lunin/lambda-calculator
cd lambda-calculator
cargo run
```

You open an REPL. Then put your lambda calculations there.

### Syntax

```ebnf

term := lambda | application | "(" term ")" | unit;

application := term term;
lambda := "\\" ident "." term;

unit := ident | literal;

ident := (* identifier *);
literal := int;
```

### Functional
TODO