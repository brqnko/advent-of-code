use std::any::Any;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!(
        "{}",
        buffer
            .trim()
            .split(",")
            .map(|s| {
                let ind = s.find("-").unwrap();
                s[0..ind].parse::<i128>().unwrap()..=s[ind + 1..].parse::<i128>().unwrap()
            })
            .map(|r| r
                .filter(|n| {
                    let string = n.to_string();
                    (2..=string.len()).any(|m| {
                        string
                            .chars()
                            .collect::<Vec<_>>()
                            .chunks(string.len() / m)
                            .fold((&string[0..string.len() / m], true), |(prev, flag), s| {
                                (prev, flag && prev == s.iter().collect::<String>())
                            })
                            .1
                    })
                })
                .sum::<i128>())
            .sum::<i128>()
    );
}
