mod matcher;

fn main() {
    let found = matcher::read_lines("fruits.txt").unwrap();
    for (idx, line) in matcher::find_matches("apple", &found) {
        println!("{}: {}", idx, line)
    }
}
