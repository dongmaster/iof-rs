# iof-rs

## What is this?
This is a crate/library that returns the index of a character/string in a string (Searching for "world" in "Hello, world!", for example.).

## How do I use iof-rs?
```rust
extern crate iof;

use iof::func::index;

fn main() {
    let index = index::of("world", "Hello, world!");
    println!("{}", index);
}
```

The function `of` takes 2 arguments as input. The needle: &str and the haystack: &str.

## What to do/improve
Add the option to start searching from a specific index (`index::of("world", "Hello, world!", 5)` the position of the number is still not set in stone).
