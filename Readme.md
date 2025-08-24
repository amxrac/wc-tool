### about
a command-line utility tool that counts bytes, lines, words, and characters in a text file.

### usage
```rust
# default: displays bytes, lines, and words
cargo run -- <filepath>

# count bytes
cargo run -- -c <filepath>

# count lines
cargo run -- -l <filepath>

# count words
cargo run -- -w <filepath>

# count characters
cargo run -- -m <filepath>
```

### Options

-c: Count bytes in file

-l: Count lines in file

-w: Count words in file

-m: Count characters in file

No flag: Default output shows bytes, lines, and words count
