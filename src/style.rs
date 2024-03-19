fn format(start: usize, end: usize) -> impl Fn(String) -> String {
    move |s| format!("\x1b[{}m{}\x1b[{}m", start, s, end,)
}

pub fn reset(s: String) -> String {
    format(0, 0)(s)
}
pub fn bold(s: String) -> String {
    format(1, 22)(s)
}
pub fn dim(s: String) -> String {
    format(2, 22)(s)
}
pub fn italic(s: String) -> String {
    format(3, 23)(s)
}
pub fn underline(s: String) -> String {
    format(4, 24)(s)
}
pub fn overline(s: String) -> String {
    format(53, 55)(s)
}
pub fn inverse(s: String) -> String {
    format(7, 27)(s)
}
pub fn hidden(s: String) -> String {
    format(8, 28)(s)
}
pub fn strikethrough(s: String) -> String {
    format(9, 29)(s)
}

pub fn black(s: String) -> String {
    format(30, 39)(s)
}
pub fn red(s: String) -> String {
    format(31, 39)(s)
}
pub fn green(s: String) -> String {
    format(32, 39)(s)
}
pub fn yellow(s: String) -> String {
    format(33, 39)(s)
}
pub fn blue(s: String) -> String {
    format(34, 39)(s)
}
pub fn magenta(s: String) -> String {
    format(35, 39)(s)
}
pub fn cyan(s: String) -> String {
    format(36, 39)(s)
}
pub fn white(s: String) -> String {
    format(37, 39)(s)
}
pub fn gray(s: String) -> String {
    format(90, 39)(s)
}

pub fn bg_black(s: String) -> String {
    format(40, 49)(s)
}
pub fn bg_red(s: String) -> String {
    format(41, 49)(s)
}
pub fn bg_green(s: String) -> String {
    format(42, 49)(s)
}
pub fn bg_yellow(s: String) -> String {
    format(43, 49)(s)
}
pub fn bg_blue(s: String) -> String {
    format(44, 49)(s)
}
pub fn bg_magenta(s: String) -> String {
    format(45, 49)(s)
}
pub fn bg_cyan(s: String) -> String {
    format(46, 49)(s)
}
pub fn bg_white(s: String) -> String {
    format(47, 49)(s)
}
pub fn bg_gray(s: String) -> String {
    format(100, 49)(s)
}

pub fn red_bright(s: String) -> String {
    format(91, 39)(s)
}
pub fn green_bright(s: String) -> String {
    format(92, 39)(s)
}
pub fn yellow_bright(s: String) -> String {
    format(93, 39)(s)
}
pub fn blue_bright(s: String) -> String {
    format(94, 39)(s)
}
pub fn magenta_bright(s: String) -> String {
    format(95, 39)(s)
}
pub fn cyan_bright(s: String) -> String {
    format(96, 39)(s)
}
pub fn white_bright(s: String) -> String {
    format(97, 39)(s)
}

pub fn bg_red_bright(s: String) -> String {
    format(101, 49)(s)
}
pub fn bg_green_bright(s: String) -> String {
    format(102, 49)(s)
}
pub fn bg_yellow_bright(s: String) -> String {
    format(103, 49)(s)
}
pub fn bg_blue_bright(s: String) -> String {
    format(104, 49)(s)
}
pub fn bg_magenta_bright(s: String) -> String {
    format(105, 49)(s)
}
pub fn bg_cyan_bright(s: String) -> String {
    format(106, 49)(s)
}
pub fn bg_white_bright(s: String) -> String {
    format(107, 49)(s)
}
