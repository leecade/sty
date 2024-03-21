// fn apply_style(start: usize, end: usize) -> impl Fn(&str) -> String {
//     move |s| format!("\x1b[{}m{}\x1b[{}m", start, s, end,)
// }
fn apply_style(s: &str, start: usize, end: usize) -> String {
    format!("\x1b[{}m{}\x1b[{}m", start, s, end)
}

pub fn reset(s: &str) -> String {
    apply_style(s, 0, 0)
}
pub fn bold(s: &str) -> String {
    apply_style(s, 1, 22)
}
pub fn dim(s: &str) -> String {
    apply_style(s, 2, 22)
}
pub fn italic(s: &str) -> String {
    apply_style(s, 3, 23)
}
pub fn underline(s: &str) -> String {
    apply_style(s, 4, 24)
}
pub fn overline(s: &str) -> String {
    apply_style(s, 53, 55)
}
pub fn inverse(s: &str) -> String {
    apply_style(s, 7, 27)
}
pub fn hidden(s: &str) -> String {
    apply_style(s, 8, 28)
}
pub fn strikethrough(s: &str) -> String {
    apply_style(s, 9, 29)
}

pub fn black(s: &str) -> String {
    apply_style(s, 30, 39)
}
pub fn red(s: &str) -> String {
    apply_style(s, 31, 39)
}
pub fn green(s: &str) -> String {
    apply_style(s, 32, 39)
}
pub fn yellow(s: &str) -> String {
    apply_style(s, 33, 39)
}
pub fn blue(s: &str) -> String {
    apply_style(s, 34, 39)
}
pub fn magenta(s: &str) -> String {
    apply_style(s, 35, 39)
}
pub fn cyan(s: &str) -> String {
    apply_style(s, 36, 39)
}
pub fn white(s: &str) -> String {
    apply_style(s, 37, 39)
}
pub fn gray(s: &str) -> String {
    apply_style(s, 90, 39)
}

pub fn bg_black(s: &str) -> String {
    apply_style(s, 40, 49)
}
pub fn bg_red(s: &str) -> String {
    apply_style(s, 41, 49)
}
pub fn bg_green(s: &str) -> String {
    apply_style(s, 42, 49)
}
pub fn bg_yellow(s: &str) -> String {
    apply_style(s, 43, 49)
}
pub fn bg_blue(s: &str) -> String {
    apply_style(s, 44, 49)
}
pub fn bg_magenta(s: &str) -> String {
    apply_style(s, 45, 49)
}
pub fn bg_cyan(s: &str) -> String {
    apply_style(s, 46, 49)
}
pub fn bg_white(s: &str) -> String {
    apply_style(s, 47, 49)
}
pub fn bg_gray(s: &str) -> String {
    apply_style(s, 100, 49)
}

pub fn red_bright(s: &str) -> String {
    apply_style(s, 91, 39)
}
pub fn green_bright(s: &str) -> String {
    apply_style(s, 92, 39)
}
pub fn yellow_bright(s: &str) -> String {
    apply_style(s, 93, 39)
}
pub fn blue_bright(s: &str) -> String {
    apply_style(s, 94, 39)
}
pub fn magenta_bright(s: &str) -> String {
    apply_style(s, 95, 39)
}
pub fn cyan_bright(s: &str) -> String {
    apply_style(s, 96, 39)
}
pub fn white_bright(s: &str) -> String {
    apply_style(s, 97, 39)
}

pub fn bg_red_bright(s: &str) -> String {
    apply_style(s, 101, 49)
}
pub fn bg_green_bright(s: &str) -> String {
    apply_style(s, 102, 49)
}
pub fn bg_yellow_bright(s: &str) -> String {
    apply_style(s, 103, 49)
}
pub fn bg_blue_bright(s: &str) -> String {
    apply_style(s, 104, 49)
}
pub fn bg_magenta_bright(s: &str) -> String {
    apply_style(s, 105, 49)
}
pub fn bg_cyan_bright(s: &str) -> String {
    apply_style(s, 106, 49)
}
pub fn bg_white_bright(s: &str) -> String {
    apply_style(s, 107, 49)
}
