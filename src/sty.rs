pub use crate::style;
pub use crate::style::*;
#[macro_export]
macro_rules! sty {
    ($text:expr, [$( $style:expr ),*]) => {{
        // allow: sty!("123", [])
        #[allow(unused_mut)]
        let mut sty_text = format!("{}", $text);
        $(
            sty_text = $style(sty_text);
        )*
        sty_text
    }};
}
