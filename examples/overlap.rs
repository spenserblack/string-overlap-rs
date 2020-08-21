use string_overlap::overlap;

fn main() {
    const BACKGROUND: &str = include_str!("../assets/passing/background.ascii");
    const FOREGROUND: &str = include_str!("../assets/passing/foreground.ascii");

    println!("Background\n{}", BACKGROUND);
    println!("Foreground\n{}", FOREGROUND);

    println!("Result\n{}", overlap(BACKGROUND, FOREGROUND));
}
