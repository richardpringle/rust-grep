use std::{
    io::{self, BufReader},
    fs::File,
};

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: io::BufRead,
{
    reader
        .lines()
        .take_while(io::Result::is_ok)
        .map(io::Result::unwrap)
        .filter(|line| line.contains(target))
        .for_each(|line_match| println!("{}", line_match));

    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);

    let pattern = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("usage: grep PATTERN [FILE_1] [FILE_2]...");
            std::process::exit(1);
        }
    };

    if args.len() == 0 {
        return grep(&pattern, io::stdin().lock()).unwrap_or_else(|e| eprintln!("{}", e));
    }

    args.for_each(|filepath| {
        let file = File::open(filepath).expect("");
        grep(&pattern, BufReader::new(file)).expect("");
    });
}
