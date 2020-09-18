use string_overlap::overlap;
use pretty_assertions::assert_eq;

#[test]
fn it_works() {
    const BACKGROUND: &str = include_str!("../assets/passing/background.ascii");
    const FOREGROUND: &str = include_str!("../assets/passing/foreground.ascii");
    const EXPECTED: &str = include_str!("../assets/expected.ascii");

    assert_eq!(overlap(BACKGROUND, FOREGROUND), EXPECTED);
}
