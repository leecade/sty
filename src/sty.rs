pub use crate::is_color_enabled;
pub use crate::set_color_enabled;
pub use crate::style;
pub use crate::style::*;

#[macro_export]
macro_rules! sty {
    ($text:expr, [$( $rule:expr ),*]) => {{
        // allow: sty!("123", [])
        #[allow(unused_mut)]
        let mut sty_text = format!("{}", $text);
        if $crate::is_color_enabled() {
            #[allow(unused_mut)]
            let mut rules: Vec<fn(&str) -> String> = Vec::new();
            $(
                rules.push($rule);
            )*
            for rule in rules.into_iter().rev() {
                sty_text = rule(&sty_text);
            }
        }
        sty_text
    }};
}
