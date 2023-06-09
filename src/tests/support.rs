pub const Q: char = '"';
pub const TAB: char = '\t';
pub const LF: char = '\n';
pub const CR: char = '\r';
pub const CRLF: &str = "\r\n";

#[macro_export]
macro_rules! bformat {
    ($($tokens:tt),*) => {
        format!($($tokens),*).into_bytes()
    };
}

#[macro_export]
macro_rules! domain_format {
    (BytesDomain, $string:literal) => {
        bformat!($string)
    };
    (CharsDomain, $string:literal) => {
        format!($string)
    };
    ($domain:ident, [ $($value:tt),* $(,)? ] $(,)?)  => {
        vec![
            $(
                domain_format!($domain, $value)
            ),*
        ]
    };
}

#[macro_export]
macro_rules! domain_format_ref {
    ($domain:ident, $string:literal) => {
        {
            use std::ops::Deref;
            domain_format!($domain, $string).deref()
        }
    };
    ($domain:ident, [ $($value:tt),* $(,)? ] $(,)?)  => {
        vec![
            $(
                domain_format_ref!($domain, $value)
            ),*
        ]
    };
}
