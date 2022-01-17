fn main() {
    println!("{} is my age ", 32);

    println!(
        "{0}, this is {1} and this {1} , and this is {0}, {0}",
        "alice", "bob"
    );

    println!(
        "{object} {verb} {subject}",
        subject = "the quick brown fox",
        object = "The lazy dog",
        verb = "jump over"
    );

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>width$}", number = 1, width = 6);
    println!("{number:`>width$}", number = 1, width = 6);

    println!("{1},{},{0},{}", 1, 2);

    let argument = 2 + 2;

    println!("{argument}");
    fn make_string(a: u32, b: &str) -> String {
        format!("{b} {a}")
    }

    let name = make_string(927, "label");

    println!("{name}");
}
