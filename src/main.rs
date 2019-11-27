use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader},
};

fn grep<R: io::BufRead>(target: &str, reader: R) -> io::Result<()> {
    reader
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .into_iter()
        .filter(|line| line.contains(target))
        .for_each(|line| println!("{}", line));

    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);

    let pattern = args.next().unwrap_or_else(|| {
        eprintln!("usage: grep PATTERN [FILE_1] [FILE_2]...");
        std::process::exit(1);
    });

    if args.len() == 0 {
        return grep(&pattern, io::stdin().lock()).unwrap_or_else(print_and_exit);
    }

    args.for_each(|filepath| {
        let file = File::open(filepath).unwrap_or_else(print_and_exit);
        grep(&pattern, BufReader::new(file)).unwrap_or_else(print_and_exit);
    });
}

fn print_and_exit<T, E: Display>(x: E) -> T {
    eprintln!("{}", x);
    std::process::exit(1);
}
