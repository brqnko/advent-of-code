fn main() -> Result<(), Box<dyn std::error::Error>> {
    // part 1
    let mut buffer = String::new();
    let mut ranges = Vec::new();
    loop {
        std::io::stdin().read_line(&mut buffer)?;
        if buffer.len() < 3 {
            break;
        }
        buffer = buffer.trim().to_string();
        let bar_idx = buffer.find("-").unwrap();
        ranges.push(buffer[0..bar_idx].parse::<u128>()?..=buffer[bar_idx + 1..].parse::<u128>()?);
        buffer.clear();
    }

    ranges.sort_by_key(|r| *r.start());

    let mut ans = 0;
    let mut rust = None;
    for range in ranges {
        if range.end() <= &rust.unwrap_or_default() {
            continue;
        }
        ans += range.end() - range.start().max(&rust.unwrap_or_default());
        if &rust.unwrap_or_default() < range.start() {
            ans += 1;
        }
        rust = Some(*range.end());
    }
    println!("{ans}");
    Ok(())
}

// 3-5 => 2
// 10-14 => 4
// 12-18 => 4
// 16-20 => 2