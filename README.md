# sty 🌈

[![Crates.io](https://img.shields.io/crates/v/sty.svg)](https://crates.io/crates/sty)
[![docs-rs](https://docs.rs/sty/badge.svg)](https://docs.rs/sty)
![Downloads](https://img.shields.io/crates/d/sty)

Color terminal outputs in a minimal, macro-based, and dead simple way.

## Highlights

- Macro-based: Increases compile-time efficiency and boosts safety.
- Innovative Styling: Compose styles in an array-like format such as `[red, bold, underline]`.
- Extensive Type Support: All types implementing `std::fmt::Display` trait are supported, offering wide-ranging adaptability.
- Full Color and Style support: Modifiers / Foreground colors / Background colors / Bright foreground colors / Bright background colors.
- Zero dependencies: Cleaner code-base with no third-party interference.

## Documentation

see [![docs-rs](https://docs.rs/sty/badge.svg)](https://docs.rs/sty)

## Installation

This crate works with Cargo. Add the following to your Cargo.toml dependencies section:

```toml
[dependencies]
sty = "0.1"
```

## Basic usage

```rust
use sty::{
    sty,
    style::{red, underline},
};
// or
// use sty::{ sty, red, underline };

println!("{}", sty!("Hello world!", [red]));

// multiple styles
println!("{}", sty!(123, [red, underline]));
```

Note: According to the ASCII escape rules, the color at the front has the highest priority. For example:

```rust
sty!("str", [red, green, blue]); // red
sty!("str", [reset, red, underline]); // reset
```

### Supported types

For any type that has implemented the `std::fmt::Display` trait, like:

- `&str`
- `String`
- `numbers`(`usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`)
- `bool`
- ...

### Supported styles

```rust
use sty::{
    style::{red, underline},
};
// or
// use sty::{ red, underline };
```

- Modifiers
  - reset
  - bold
  - dim
  - italic
  - underline
  - overline
  - inverse
  - hidden
  - strikethrough
- Foreground colors
  - black
  - red
  - green
  - yellow
  - blue
  - magenta
  - cyan
  - white
  - gray
- Background colors
  - bg_black
  - bg_red
  - bg_green
  - bg_yellow
  - bg_blue
  - bg_magenta
  - bg_cyan
  - bg_white
  - bg_gray
- Bright foreground colors
  - red_bright
  - green_bright
  - yellow_bright
  - blue_bright
  - magenta_bright
  - cyan_bright
  - white_bright
- Bright background colors
  - bg_red_bright
  - bg_green_bright
  - bg_yellow_bright
  - bg_blue_bright
  - bg_magenta_bright
  - bg_cyan_bright
  - bg_white_bright
