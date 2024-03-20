use sty::{sty, style::*};
fn main() {
    println!(
        "   {}   {}   {}   {}   {}   {}   {}   {}   {}   ",
        sty!("black", [black]),
        sty!("red", [red]),
        sty!("green", [green]),
        sty!("yellow", [yellow]),
        sty!("blue", [blue]),
        sty!("magenta", [magenta]),
        sty!("cyan", [cyan]),
        sty!("white", [white]),
        sty!("gray", [gray])
    );

    println!(
        "{}{}{}{}{}{}{}{}{}",
        sty!("bg_black", [bg_black]),
        sty!("bg_red", [bg_red]),
        sty!("bg_green", [bg_green]),
        sty!("bg_yellow", [bg_yellow]),
        sty!("bg_blue", [bg_blue]),
        sty!("bg_magenta", [bg_magenta]),
        sty!("bg_cyan", [bg_cyan]),
        sty!("bg_white", [bg_white]),
        sty!("bg_gray", [bg_gray])
    );

    println!(
        "{}{}{}{}{}{}{}",
        sty!("bg_red_bright", [bg_red_bright]),
        sty!("bg_green_bright", [bg_green_bright]),
        sty!("bg_yellow_bright", [bg_yellow_bright]),
        sty!("bg_blue_bright", [bg_blue_bright]),
        sty!("bg_magenta_bright", [bg_magenta_bright]),
        sty!("bg_cyan_bright", [bg_cyan_bright]),
        sty!("bg_white_bright", [bg_white_bright])
    );

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        sty!("red_bright", [red_bright]),
        sty!("green_bright", [green_bright]),
        sty!("yellow_bright", [yellow_bright]),
        sty!("blue_bright", [blue_bright]),
        sty!("magenta_bright", [magenta_bright]),
        sty!("cyan_bright", [cyan_bright]),
        sty!("white_bright", [white_bright])
    );

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        sty!("reset", [reset]),
        sty!("bold", [bold]),
        sty!("dim", [dim]),
        sty!("italic", [italic]),
        sty!("underline", [underline]),
        sty!("overline", [overline]),
        sty!("inverse", [inverse]),
        sty!("hidden", [hidden]),
        sty!("strikethrough", [strikethrough])
    );
}
