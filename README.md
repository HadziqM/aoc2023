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
```
mycli new 1
```
in the folder day<daynumber>, write your own solution and if you want to automatically answer them just use this snippet in the code
```rust
Common::answer(day:u8,part:u8,answer:String)
```
to run from workplace dir just use
```shell
cargo run --package day1
```
or simply
```shell
mycli run 1
```
