## Dummy own Cli for advent of code

insatall cli by
```shell
cargo install --path ./mycli
```

scaffold puzzle + get input with
```shell
mycli new <day number>
```
for example at day 1
```chell
mycli new 1
```
just dont forget to rename `session.json.example` into `sesson.json` and fill the cookie from advent of code cookie on dev tool </br>

if you somehow cant get the input because invalid cookie you can try again to download the input with
```shell
mycli input 1
```
if you somehow cant get the question or wanna get the second part of the puzzle after clearing first half you can try again to download the question with
```shell
mycli question 1
```

in the folder day<daynumber>, write your own solution and if you want to automatically answer them just use this snippet in the code
```rust
puzzle.answer(part:u8,answer:String)
```
to run from workplace dir just use
```shell
cargo run --package day1
```
or simply
```shell
mycli run 1
```
