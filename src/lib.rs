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
//!
//! # Features
//!
//! ## Default
//!
//! ### `colored`
//!
//! Allows overlapping of `ColoredString`s from the [colored] crate.
//!
//! [colored]: https://crates.io/crates/colored
#[cfg(feature = "colored")]
use colored_crate::ColoredString;

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

/// Overlap `ColoredString`s.
///
/// # Example
///
/// ```rust
/// # extern crate colored_crate as colored;
/// use colored::Colorize;
/// use string_overlap::overlap_colored;
///
/// let background = "\
/// ...
/// ...
/// ...".red();
/// let foreground = "\
/// foo
///   o
///   f".blue();
///
/// assert_eq!(
///     overlap_colored(background, foreground),
///     format!(
///         "{line1}\n{line2}\n{line3}\n",
///         line1="foo".blue(),
///         line2=format!("{}{}", "..".red(), "o".blue()),
///         line3=format!("{}{}", "..".red(), "f".blue()),
///     ),
/// );
/// ```
#[cfg(feature = "colored")]
pub fn overlap_colored(background: ColoredString, foreground: ColoredString) -> String {
    use colored_crate::Colorize;
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref NOT_WHITESPACE: Regex = Regex::new(r"[^\s]+").unwrap();
    }

    let background_bg = background.bgcolor();
    let background_fg = background.fgcolor();
    let _background_style = background.style();
    let foreground_bg = foreground.bgcolor();
    let foreground_fg = foreground.fgcolor();
    let _foreground_style = foreground.style();

    let background = background.lines();
    let foreground = foreground.lines();

    background
        .zip_longest(foreground)
        .map(|lines| {
            let combined_line: String = match lines {
                Both(b_line, f_line) => {
                    let fg_matches = NOT_WHITESPACE.find_iter(f_line);

                    let mut background_left_boundary = 0;
                    let mut line = String::new();

                    for m in fg_matches {
                        let b_left = b_line.get(background_left_boundary..m.start());
                        background_left_boundary = m.end();
                        match b_left {
                            Some(s) => {
                                if s.len() > 0 {
                                    let s = match background_fg {
                                        Some(color) => s.color(color),
                                        None => s.into(),
                                    };
                                    let s = match background_bg {
                                        Some(color) => s.on_color(color),
                                        None => s,
                                    };
                                    line.push_str(&s.to_string());
                                }
                            }
                            None => {}
                        }
                        let fg = m.as_str();
                        let fg = match foreground_fg {
                            Some(color) => fg.color(color),
                            None => fg.into(),
                        };
                        let fg = match foreground_bg {
                            Some(color) => fg.on_color(color),
                            None => fg,
                        };
                        line.push_str(&fg.to_string());
                    }

                    let final_part = b_line.get(background_left_boundary..);
                    match final_part {
                        Some(s) => {
                            if s.len() > 0 {
                                let s = match background_fg {
                                    Some(color) => s.color(color),
                                    None => s.into(),
                                };
                                let s = match background_bg {
                                    Some(color) => s.on_color(color),
                                    None => s,
                                };
                                line.push_str(&s.to_string());
                            }
                        }
                        None => {}
                    }
                    line.push('\n');
                    line
                }
                Left(b_line) => {
                    let b_line = match background_fg {
                        Some(color) => b_line.color(color),
                        None => b_line.into(),
                    };
                    let b_line = match background_bg {
                        Some(color) => b_line.on_color(color),
                        None => b_line,
                    };
                    b_line.to_string()
                }
                Right(f_line) => {
                    let f_line = match foreground_fg {
                        Some(color) => f_line.color(color),
                        None => f_line.into(),
                    };
                    let f_line = match foreground_bg {
                        Some(color) => f_line.on_color(color),
                        None => f_line,
                    };
                    f_line.to_string()
                }
            };
            combined_line
        })
        .collect()
}
