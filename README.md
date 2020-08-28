# string-overlap

[![Crates.io](https://img.shields.io/crates/v/string-overlap)](https://crates.io/crates/string-overlap)
[![Docs.rs](https://docs.rs/string-overlap/badge.svg)](https://docs.rs/string-overlap)
![Crates.io](https://img.shields.io/crates/d/string-overlap)
[![Build Status](https://travis-ci.com/spenserblack/string-overlap-rs.svg?branch=master)](https://travis-ci.com/spenserblack/string-overlap-rs)

A helper crate for "layering" ASCII art

## Example

*The below mirrors [`cargo run --example overlap`](https://github.com/spenserblack/string-overlap-rs/blob/master/examples/overlap.rs).*

### Input

#### Background

```text
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
```

#### Foreground

```text
FFFFFFFFFF
FFFFFFFFFF
FF
FF
FFFFFFF
FFFFFFF
FF
FF
FF
FF
```

### Output

```text
FFFFFFFFFF
FFFFFFFFFF
FF........
FF........
FFFFFFF...
FFFFFFF...
FF........
FF........
FF........
FF........
```
