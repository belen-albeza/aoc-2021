# aoc-2021

Advent of Code 2021 with Rust.

## To build and run the project

This project uses [cargo-aoc](https://github.com/gobanos/cargo-aoc). More detailed instructions can be found at that project's [README](https://github.com/gobanos/cargo-aoc/blob/master/README.md) file.

1. Create an account at adventofcode.com
2. Get the value for your session cookie and configure `cargo-aoc`:

```
cargo aoc credentials -s TOKEN
```

3. Build and run the code with:

```
cargo aoc
```

## Log

### Day 1

I found out about the [`slice::windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) method in Rust, which enables both parts of today's puzzle to be solved with functional programming.

For **part 1**, it's as simple as iterating the entries with a window of size `2`, and then folding the result:

```rust
measurements.windows(2).fold(0, |total, window| {
  total + if window[1] > window[0] { 1 } else { 0 }
})
```

For **part2** we need to first create triplets (which are windows of size `3`), and then iterate over the list of triplets as if they were windows of size `2`, to be able to access both the current triplet and the previous one:

```rust
let triplets: Vec<&[u64]> = measurements.windows(3).collect();
triplets.windows(2).fold(0, |total, window| {
  // ...
})
```

### Day 2

I created an enum type to handle the different commands. This implements the trait `std::convert::From` to parse a string into a command. Overengineering? Yes, but… 🤷🏻‍♀️

```rust
pub enum Command {
  Forward(i64),
  Up(i64),
  Down(i64),
}

impl From<&str> for Command {
  fn from(raw: &str) -> Self {
    // ...
  }
}
```

For **part 1**, running the commands is as simple as do a `match` over that enum:

```rust
fn exec(&mut self, cmd: &Command) {
  match cmd {
    Command::Forward(delta) => self.x += delta,
    Command::Up(delta) => self.y -= delta,
    Command::Down(delta) => self.y += delta,
  }
}
```

For **part 2**, a new logic for handling the commands is introduced, plus a new variable into the state of the submarine (`aim`). So I decided to play with Rust traits a bit and have two different types for the different "submarines", each one implementing a trait that provides the `exec` method:

```rust
pub trait Submarine {
  fn exec(&mut self, cmd: &Command);
  fn run(&mut self, input: &[Command]) {
    // ...
  }
}

pub struct SubmarineV2 {
  x: i64,
  y: i64,
  aim: i64,
}

impl Submarine for SubmarineV2 {
  fn exec(&mut self, cmd: &Command) {
    // ...
  }
}
```
