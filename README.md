# Usage

```
$ cargo build

$ ./target/debug/rtsq --language cpp --query "(return_statement) @foo" ./examples
./examples/helloworld.cpp:4 matches foo
```