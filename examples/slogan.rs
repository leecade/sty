use sty::{sty, style::*};
fn main() {
    println!(
        "\n{}",
        sty!(
            format!(
                "{} {} {} {} {}, {}, {} {} {}",
                sty!("Style", [red]),
                sty!("terminal", [white, bg_black]),
                sty!("outputs", [black]),
                sty!("in a", [gray]),
                sty!("minimal", [magenta]),
                sty!("macro-based", [yellow_bright]),
                sty!("and", [gray]),
                sty!("dead simple", [green_bright]),
                sty!("way.", [gray]),
            ),
            [underline, italic]
        )
    );
}
