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

### Day 3

Today's puzzle was a good opportunity to refresh bit shifts and masking in Rust.

For **part 1**, the key of my solution is this loop:

```rust
for i in 0..n_bits {
  let mask = 2_u32.pow(i);
  let ones = report.iter().filter(|x| *x & mask == mask).count();
  if ones > report.len() / 2 {
      gamma_rate += mask
  }
}
```

Let's say we want to get the value of the second bit (counting from the left) for a number, for instance `01110`. We would need to build a mask that is `01000` so when we do and `AND` operation with those two we can get whichever value was at the position of the masking bit.

For **part 2**, we use the same masking idea and keep filtering down the numbers until we have only one left.

### Day 4

This was more labor-intensive that previous days, but at least it can be solved without having to implement any optimizations.

To handle the bingo cards, I opted for a `Card` struct and a `Number` enum (which variants for marked an unmarked numbers). `Card` just contains a static array of 25 elements to represent the numbers.

```rust
pub enum Number {
  Unmarked(u64),
  Marked(u64),
}
// ...
pub struct Card {
  numbers: [Number; CARD_SIZE * CARD_SIZE],
}
```

Given an index in that `numbers` array, we can easily get its corresponding row and column:

```rust
let col = i % CARD_SIZE;
let row = i / CARD_SIZE;
```

In **part 1**, we just need to get the first card which has a bingo. I implemented a method that can check for bingo for given range and step. For checking a row the step is `1`, since we are just looking at consecutive elements in the array. For checking a column, the step is `5`.

In **part 2** the only different thing is to get the _last_ card that scores a bingo. Since apparently there's no convenient method in Rust to remove multiple indices from a `Vec` at once, I just opted to add winner cards to an ignore list, so they are not re-checked for bingo.

### Day 5

Instead of virtually "drawing" or populating a grid with the points in question, I kept a list of segments and implemented a `points()` method in them that would return all the points in the grid the segment would fill.

This would be super easy in any language, but I found out the hard way that Rust ranges _do not support a negative step_.

My code is kind of convoluted and very verbose. I'm not exactly happy with my solution to this problem, but at least it worked.

### Day 6

**Part 1** was simple, I just had an array for all the laternfish, and pushed to it when it was time for one to reproduce.

```rust
fn simulation(input: &[u64], n: u64) -> u64 {
  let mut population = input.to_vec();
  for _ in 0..n {
    population = tick(&population);
  }

  population.len() as u64
}

fn tick(fishes: &[u64]) -> Vec<u64> {
  let mut population = vec![];

  for fish in fishes.iter() {
    if *fish == 0 {
      population.push(6); // reset counter
      population.push(8); // new born fish
    }
    else {
      population.push(fish - 1) // decrease counter
    }
  }

  population
}
```

However, with **part 2**, which was exactly the same problem but with more timespan for the lanternfish population, it required optimization.

I decided to opt for a classic [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming) technique, in which I could split the result into partials that could in turn be memoized. To get that split, instead of processing every tick the whole population of laternfish, I had to do the reverse: processing each fish for the whole timespan.

```rust
fn simulation(input: &[u64], n: u64) -> u64 {
  // ...
  for fish in input.to_vec().into_iter() {
    population_count += simulate_fish(fish as i64, n as i64, &mut cache);
  }
  // ...
}
```

Then, to simulate fish I just needed to calculate when it would reproduce and how much timespan would be available to its children. This `simulate_fish` function is provided a "cache" in the form of a `HashMap` to store its partial results.

```rust
fn simulate_fish(fish: i64, n: i64, cache: &mut HashMap<(i64, i64), u64>) -> u64 {
  // ...
  for i in (fish..n+fish).step_by(7) {
    fish_count += simulate_fish(8, n-i-1, cache);
  }
  // ...
}
```

### Day 7

TODO

### Day 8

I found the wording of the puzzle quite confusing for today. At any case, I solved **part 1** in a breeze. I just filtered the target lengths and reduced into a sum:

```rust
let targets = [2, 4, 3, 7];
input
  .into_iter()
  .map(|(_, output)| output.clone())
  .flatten()
  .fold(0, |total, x| {
    total + if targets.contains(&x.len()) { 1 } else { 0 }
  })
```

I didn't find **part 2** particularly difficult, _but_. So verbose 😭. Here I struggled with casting types all the time.

The key part was the bahamut of a function that I did to create a dictionary of a mangled digit string to its numeric value.

I hardcoded all the logical rules:

1. Identify straight away `1`, `4`, `7` and `8` because of their unique length.
2. Find out to which mangled character corresponds which segment of the display.
3. Identify straight away segments `b`, `e` and `f` because of their unique frequency in all the digits.
4. Segment `c` is the one in `1` that is not `f` (which we already know).
5. Segment `d` is the one in `4` that is not `b`, `c`, or `f`.
6. ...
7. Identify the remaining unknown digits by checking against the known segments.

Then, after that, it was only a matter of converting a list of digits into an actual integer we could sum to provide the result.
