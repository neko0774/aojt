use std::env::args;

fn main() {
    println!("Hello aojt!");
    struct Cli{
        pattern: String,
        path: std::path::PathBuf,
    }
    let pattern = args().nth(1).expect("not given the pattern");
    let path = args().nth(2).expect("not given the path");
    let args = Cli{
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
}
