#![doc = include_str!("../README.md")]

pub mod sty;
pub mod style;
pub use style::*;

use once_cell::sync::Lazy;
use std::sync::RwLock;

#[rustversion::since(1.70)]
fn detect_tty_support() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}

#[rustversion::before(1.70)]
fn detect_tty_support() -> bool {
    // TODO: Enhanced detection
    if let Ok(term) = std::env::var("TERM") {
        term.contains("color") || term.contains("256")
    } else {
        false
    }
}

// pub static SUPPORTS_COLOR: Lazy<bool> = Lazy::new(|| detect_tty_support());
static SUPPORTS_COLOR: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(detect_tty_support()));

pub fn set_color_enabled(enable: bool) {
    let mut color_support = match SUPPORTS_COLOR.write() {
        Ok(guard) => guard,
        Err(_) => {
            return;
        }
    };
    *color_support = enable;
}

pub fn is_color_enabled() -> bool {
    match SUPPORTS_COLOR.read() {
        Ok(guard) => *guard,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use once_cell::sync::Lazy;
    // use std::sync::Mutex;
    // static COLOR_SUPPORT_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[test]
    fn test_color_enable() {
        // let _guard = COLOR_SUPPORT_MUTEX.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50));
        let enabled = is_color_enabled();
        set_color_enabled(true);
        assert_eq!(sty!("123", [red]), "\u{1b}[31m123\u{1b}[39m");
        set_color_enabled(false);
        assert_eq!(sty!("123", [red]), "123");
        set_color_enabled(enabled);
    }

    #[test]
    fn test_basic_usage() {
        assert_eq!(sty!("123", []), "123");
        assert_eq!(sty!("123", [red]), "\u{1b}[31m123\u{1b}[39m");
    }

    #[test]
    fn test_standlone_rules() {
        assert_eq!(red("red"), "\u{1b}[31mred\u{1b}[39m");
        assert_eq!(
            red(&underline("red")),
            "\u{1b}[31m\u{1b}[4mred\u{1b}[24m\u{1b}[39m"
        );
    }

    #[test]
    fn test_combination_rules() {
        assert_eq!(sty!("123", []), "123");
        assert_eq!(
            sty!("123", [red, green]),
            "\u{1b}[31m\u{1b}[32m123\u{1b}[39m\u{1b}[39m"
        );
        assert_eq!(
            sty!("456", [red, underline]),
            "\u{1b}[31m\u{1b}[4m456\u{1b}[24m\u{1b}[39m"
        );
        assert_eq!(
            sty!("!reset", [reset, red, underline]),
            "\u{1b}[0m\u{1b}[31m\u{1b}[4m!reset\u{1b}[24m\u{1b}[39m\u{1b}[0m"
        );
        assert_eq!(
            sty!("reset", [red, underline, reset]),
            "\u{1b}[31m\u{1b}[4m\u{1b}[0mreset\u{1b}[0m\u{1b}[24m\u{1b}[39m"
        );
    }
    #[test]
    fn test_import() {
        assert_eq!(sty!("123", [red]), sty!("123", [style::red]));
    }
    #[test]
    fn test_more_types() {
        // &str
        assert_eq!(sty!("str", [red]), "\u{1b}[31mstr\u{1b}[39m");

        // String
        assert_eq!(
            sty!("String".to_string(), [red]),
            "\u{1b}[31mString\u{1b}[39m"
        );

        // usize
        assert_eq!(sty!(123, [red]), "\u{1b}[31m123\u{1b}[39m");

        // float
        assert_eq!(sty!(123.123, [red]), "\u{1b}[31m123.123\u{1b}[39m");

        // ifloat
        assert_eq!(sty!(-123.123, [red]), "\u{1b}[31m-123.123\u{1b}[39m");
    }
}
