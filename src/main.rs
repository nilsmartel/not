use std::io::Read;

fn main() {
    let pattern = match std::env::args().nth(1) {
        None => print_help(),
        Some(x) if x == "--help" => print_help(),
        Some(pattern) => pattern,
    };

    let regex = match regex::Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("invalid pattern. {:#?}", e);
            std::process::exit(1)
        }
    };


    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("to read stdin as string");

    for line in input.lines() {
        if !regex.is_match(line) {
            println!("{}", line);
        }
    }
}

fn print_help() -> ! {
    eprintln!("not <PATTERN>

        Filter lines from stdin,
        by removing them if they
        fit the <PATTERN>.
        ");
    std::process::exit(0)
}
