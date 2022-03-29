mod structs;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = structs::commands::Command{
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
}
