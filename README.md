# sty 🌈

[![Crates.io](https://img.shields.io/crates/v/sty.svg)](https://crates.io/crates/sty)
[![docs-rs](https://docs.rs/sty/badge.svg)](https://docs.rs/sty)
![Downloads](https://img.shields.io/crates/d/sty)

## $\mathbb{\color{red}{Style \ } \color{lightblue}{terminal}\ \color{black}{outputs \ }\color{gray}{\ in\ a} \color{magenta}{\ minimal}\color{gray}{,} \color{lightyellow}{\ macro\ based}\color{gray}{,}\ and \color{lightgreen}{\ dead\ simple} \color{gray}{\ way.}}$

|                               |   Code    |  Backgroud   |      Bright      |  Bright background  |
| ----------------------------- | :-------: | :----------: | :--------------: | :-----------------: |
| $\mathbb{\color{black}{■}}$   |  `black`  |  `bg_black`  |                  |                     |
| $\mathbb{\color{gray}{■}}$    |  `gray`   |  `bg_gray`   |                  |                     |
| $\mathbb{\color{white}{■}}$   |  `white`  |  `bg_white`  |  `white_bright`  |  `bg_white_bright`  |
| $\mathbb{\color{red}{■}}$     |   `red`   |   `bg_red`   |   `red_bright`   |   `bg_red_bright`   |
| $\mathbb{\color{green}{■}}$   |  `green`  |  `bg_green`  |  `green_bright`  |  `bg_green_bright`  |
| $\mathbb{\color{yellow}{■}}$  | `yellow`  | `bg_yellow`  |  `yello_bright`  | `bg_yellow_bright`  |
| $\mathbb{\color{blue}{■}}$    |  `blue`   |  `bg_blue`   |  `blue_bright`   |  `bg_blue_bright`   |
| $\mathbb{\color{magenta}{■}}$ | `magenta` | `bg_magenta` | `magenta_bright` | `bg_magenta_bright` |
| $\mathbb{\color{cyan}{■}}$    |  `cyan`   |  `bg_cyan`   |  `cyan_bright`   |  `bg_cyan_bright`   |

> **`bold`** _`italic`_ <ins>`underline`</ins> <s>`strikethrough`</s> `overline` `dim` `inverse` `hidden` `reset`

## Highlights

- **Macro-based**: Increases compile-time efficiency and boosts safety.
- **Innovative Styling**: Compose styles in an array-like format such as `[red, bold, underline]`.
- **Extensive Type Support**: All types implementing `std::fmt::Display` trait are supported, offering wide-ranging adaptability.
- **Full Color and Style support**: Modifiers / Foreground colors / Background colors / Bright foreground colors / Bright background colors.
- **Zero dependencies**: Cleaner code-base with no third-party interference.
- **Supports `tty` detection**: Avoids styling when the tty lacks color support, Use `set_color_enabled` for manual enable control.

## Documentation

[![docs-rs](https://docs.rs/sty/badge.svg)](https://docs.rs/sty)

## Installation

This crate works with Cargo. Add the following to your Cargo.toml dependencies section:

```toml
[dependencies]
sty = "0.3"
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

// Use `sty!` macro for combination of multiple styles
println!("{}", sty!("Hello world!", [red, underline]));

// Use `sty!` macro for multiple input types
println!("{}", sty!(123, [red, underline]));

// Use style function is simple for `&str` input
println!("{}", red("Hello world!"));
println!("{}", underline(&red("Hello world!")));
```

> [!TIP]
> In style combinations, the styles that are specified later take precedence, For example:

```rust
use sty::{ sty, style::* };
sty!("str", [red, green, blue]); // blue
sty!("str", [red, underline, reset]); // reset
```

> [!TIP]
> If `tty` environment does not support color, default output will be printed without any styling, Use `set_color_enabled` for manual enable control, For example:

```rust
use sty::{ set_color_enabled, is_color_enabled, sty, style::* };
let enabled = is_color_enabled();
set_color_enabled(false);
sty!("123", [red]); // no color
set_color_enabled(true);
sty!("123", [red]); // red
set_color_enabled(enabled);
```

### `sty!` Macro

The `sty!` macro is a powerful utility conveniently used to apply a collection of style manipulations to your input. This macro allows you to seamlessly incorporate text properties with an intuitive syntax.

```rust
use sty::{ sty, style::{red, underline} };
println!("{}", sty!("Hello world!", [red, underline]));
```

In this example, `red` and `underline` are both style transformations, and the sty! macro returns a newly styled text instance with all these styling elements applied. Multiple styles can be applied simultaneously, and the output will be a combination of all.

#### Supported input types

For any type that has implemented the `std::fmt::Display` trait, like:

- `&str`
- `String`
- `numbers`(`usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`)
- `bool`
- ...

### Style function

Each style function can be combine with `sty!` macro or used separately, for example:

```rust
use sty::{
    sty,
    style::{red, underline},
};
// or
// use sty::{ sty, red, underline };

sty!("Hello world!", [red, underline]);
// or
red("Hello world!");
```

> [!TIP]
> The standalone style function only supports `&str` input type.

All style functions are as follows:

- Modifiers
  - `reset`
  - `bold`
  - `dim`
  - `italic`
  - `underline`
  - `overline`
  - `inverse`
  - `hidden`
  - `strikethrough`
- Foreground colors
  - `black`
  - `red`
  - `green`
  - `yellow`
  - `blue`
  - `magenta`
  - `cyan`
  - `white`
  - `gray`
- Background colors
  - `bg_black`
  - `bg_red`
  - `bg_green`
  - `bg_yellow`
  - `bg_blue`
  - `bg_magenta`
  - `bg_cyan`
  - `bg_white`
  - `bg_gray`
- Bright foreground colors
  - `red_bright`
  - `green_bright`
  - `yellow_bright`
  - `blue_bright`
  - `magenta_bright`
  - `cyan_bright`
  - `white_bright`
- Bright background colors

  - `bg_red_bright`
  - `bg_green_bright`
  - `bg_yellow_bright`
  - `bg_blue_bright`
  - `bg_magenta_bright`
  - `bg_cyan_bright`
  - `bg_white_bright`

## Contribution

[CONTRIBUTING](CONTRIBUTING.md)
