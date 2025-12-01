use std::io::BufRead;

fn main() -> Result<(), &'static str> {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .fold((0usize, 50i128), |(ans, dial), line| {
                let count = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<i128>()
                    .unwrap_or_default();
                match &line[0..1] {
                    "L" => (
                        ans + (dial + 1..=dial + count).filter(|n| n % 100 == 0).count(),
                        dial + count,
                    ),
                    "R" => (
                        ans + (dial - count..dial).filter(|n| n % 100 == 0).count(),
                        dial - count,
                    ),
                    _ => panic!("unexepted direction"),
                }
            })
            .0
    );
    Ok(())
}
