use colored_crate::Colorize;
use string_overlap::overlap_colored;

const BACKGROUND: &str = include_str!("../assets/flag/background.ascii");
const FOREGROUND: &str = include_str!("../assets/flag/foreground.ascii");

fn main() {
    println!("plain:\n{}", overlap_colored(BACKGROUND.into(), FOREGROUND.into()));
    println!("blue on plain:\n{}", overlap_colored(BACKGROUND.into(), FOREGROUND.blue()));
    println!("plain on red:\n{}", overlap_colored(BACKGROUND.red(), FOREGROUND.into()));
    println!("yellow on blue:\n{}", overlap_colored(BACKGROUND.blue(), FOREGROUND.yellow()));
}
