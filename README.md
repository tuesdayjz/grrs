### grrs - grep-like tool in Rust

reffered to the book: [Command Line Applications in Rust](https://rust-cli.github.io/book/)

#### Usage

once you run `cargo build` under the project directory, you will find the binary file `grrs` in the `target/debug` directory.

```shell
$ grrs <pattern> <path>
```

also you can run the binary file directly by `cargo run <pattern> <path>`.

#### Example

```shell
$ grrs src/main.rs fn

18: fn main() {
38: fn file_grepper<R: BufRead>(pattern: &str, buf_reader: R) -> std::io::Result<()> {
```
