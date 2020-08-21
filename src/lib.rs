use std::fmt::Display;

pub fn overlap<B, F>(background: B, foreground: F) -> String where B: Display, F: Display {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::overlap;

    #[test]
    fn it_works() {
        const BACKGROUND: &str = include_str!("../assets/passing/background.ascii");
        const FOREGROUND: &str = include_str!("../assets/passing/foreground.ascii");
        const EXPECTED: &str = include_str!("../assets/expected.ascii");

        assert_eq!(overlap(BACKGROUND, FOREGROUND), EXPECTED);
    }
}
