use colored_crate::{ColoredString, Colorize};
use indoc::indoc;
use pretty_assertions::assert_eq;
use string_overlap::overlap_colored;

const BACKGROUND: &str = include_str!("../assets/flag/background.ascii");
const FOREGROUND: &str = include_str!("../assets/flag/foreground.ascii");

#[test]
fn plain_on_plain() {
    const EXPECTED: &str = indoc!{"
        ..................|||.................
        ..................|||.................
        ..................|||.................
        ..................|||.................
        ||||||||||||||||||||||||||||||||||||||
        ||||||||||||||||||||||||||||||||||||||
        ..................|||.................
        ..................|||.................
        ..................|||.................
        ..................|||.................
    "};

    let background = BACKGROUND.into();
    let foreground = FOREGROUND.into();
    let actual = overlap_colored(background, foreground).to_string();

    assert_eq!(
        actual,
        EXPECTED,
        "got\n{actual}\nexpected\n{expected}",
        actual=actual,
        expected=EXPECTED,
    );
}

#[test]
fn plain_on_colored() {
    let expected = {
        let vertical_line = format!(
            "{left}{middle}{right}\n",
            left="..................".red(),
            middle=ColoredString::from("|||"),
            right=".................".red(),
        );
        let horizontal_line: ColoredString = "||||||||||||||||||||||||||||||||||||||".into();
        let horizontal_line = format!("{}\n", horizontal_line);
        format!(
            "{top}{middle}{bottom}",
            top=vertical_line.repeat(4),
            middle=horizontal_line.repeat(2),
            bottom=vertical_line.repeat(4),
        )
    };
    let background = BACKGROUND.red();
    let foreground = FOREGROUND.into();
    let actual = overlap_colored(background, foreground).to_string();

    assert_eq!(
        actual,
        expected,
        "got\n{actual}\nexpected\n{expected}",
        actual=actual,
        expected=expected,
    );
}

#[test]
fn colored_on_plain() {
    let expected = {
        let vertical_line = format!(
            "{left}{middle}{right}\n",
            left=ColoredString::from(".................."),
            middle="|||".blue(),
            right=ColoredString::from("................."),
        );
        let horizontal_line = "||||||||||||||||||||||||||||||||||||||".blue();
        let horizontal_line = format!("{}\n", horizontal_line);
        format!(
            "{top}{middle}{bottom}",
            top=vertical_line.repeat(4),
            middle=horizontal_line.repeat(2),
            bottom=vertical_line.repeat(4),
        )
    };
    let background = BACKGROUND.into();
    let foreground = FOREGROUND.blue();
    let actual = overlap_colored(background, foreground).to_string();

    assert_eq!(
        actual,
        expected,
        "got\n{actual}\nexpected\n{expected}",
        actual=actual,
        expected=expected,
    );
}

#[test]
fn colored_on_colored() {
    let expected = {
        let vertical_line = format!(
            "{left}{middle}{right}\n",
            left="..................".blue(),
            middle="|||".yellow(),
            right=".................".blue(),
        );
        let horizontal_line = "||||||||||||||||||||||||||||||||||||||".yellow();
        let horizontal_line = format!("{}\n", horizontal_line);
        format!(
            "{top}{middle}{bottom}",
            top=vertical_line.repeat(4),
            middle=horizontal_line.repeat(2),
            bottom=vertical_line.repeat(4),
        )
    };
    let background = BACKGROUND.blue();
    let foreground = FOREGROUND.yellow();
    let actual = overlap_colored(background, foreground).to_string();

    assert_eq!(
        actual,
        expected,
        "got\n{actual}\nexpected\n{expected}",
        actual=actual,
        expected=expected,
    );
}
