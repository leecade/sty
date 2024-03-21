use sty::{
    sty,
    style::{red, underline},
};
fn main() {
    println!("{} {}", sty!("Hello", [red]), underline("world!"));
}
