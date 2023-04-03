use std::io::{self, Write};

macro_rules! r_line {
    () => {{
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read from stdin");
        line.trim_end_matches(&['\n', '\r']).to_string()
    }};
}

macro_rules! r_int {
    () => {
        r_line!().parse::<i32>().unwrap()
    };
}

macro_rules! r_f64 {
    () => {
        r_line!().parse::<f64>().unwrap()
    };
}

fn main() {
    println!("\n    [rust]: What's your name, stranger?");
    print!("[stranger]: ");
    io::stdout().flush().unwrap();
    let name = r_line!();
    println!("    [rust]: Hello, {}!\n", name);
}
