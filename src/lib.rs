//! Overlap text.
//!
//! For overlapping purposes, whitespace characters in the foreground are
//! treated as "invisible," and the character from the background will be
//! used instead.
//!
//! *__NOTE__ Final newlines are inserted.*
//!
//! # Example
//!
//! ```rust
//! use string_overlap::overlap;
//!
//! let background = "\
//! ...
//! ...
//! ...";
//! let foreground = "\
//! foo
//!   o
//!   f";
//!
//! assert_eq!(overlap(background, foreground), "\
//! foo
//! ..o
//! ..f\n");
//! ```
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::fmt::Display;

/// Places `foreground` "on top of" `background`.
pub fn overlap<B, F>(background: B, foreground: F) -> String
where
    B: Display,
    F: Display,
{
    let background = background.to_string();
    let foreground = foreground.to_string();
    let background = background.lines();
    let foreground = foreground.lines();

    background
        .zip_longest(foreground)
        .map(|lines| {
            let combined_line: String = match lines {
                Both(b_line, f_line) => {
                    let b_chars = b_line.chars();
                    let f_chars = f_line.chars();

                    let combined_line: String = b_chars
                        .zip_longest(f_chars)
                        .map(|chars| match chars {
                            Both(b_char, f_char) if f_char.is_whitespace() => b_char,
                            Both(_, f_char) => f_char,
                            Left(b_char) => b_char,
                            Right(f_char) => f_char,
                        })
                        .collect();
                    combined_line
                }
                Left(b_line) => b_line.to_string(),
                Right(f_line) => f_line.to_string(),
            };
            format!("{}\n", combined_line)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::overlap;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        const BACKGROUND: &str = include_str!("../assets/passing/background.ascii");
        const FOREGROUND: &str = include_str!("../assets/passing/foreground.ascii");
        const EXPECTED: &str = include_str!("../assets/expected.ascii");

        assert_eq!(overlap(BACKGROUND, FOREGROUND), EXPECTED);
    }
}
