#![doc = include_str!("../README.md")]

pub mod sty;
pub mod style;
pub use style::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_usage() {
        assert_eq!(sty!("123", []), "123");
        assert_eq!(sty!("123", [red]), "\u{1b}[31m123\u{1b}[39m");
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
