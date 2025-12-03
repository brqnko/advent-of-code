use std::{collections::HashSet, io::BufRead};

fn main() {
    // part 1
    /*println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|line| {
                let numbers = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or_default())
                    .collect::<Vec<_>>();
                (0..numbers.len())
                    .map(|comb_1| (comb_1 + 1..numbers.len()).map(move |comb_2| (comb_1, comb_2)))
                    .flatten()
                    .map(|comb| numbers[comb.0] * 10 + numbers[comb.1])
                    .max()
                    .unwrap()
            })
            .sum::<u32>()
    );*/

    // part 2, (TLE)
    /*println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|line| {
                let numbers = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or_default() as u128)
                    .collect::<Vec<_>>();
                (0..1 << numbers.len())
                    .map(|bit| {
                        (0..numbers.len())
                            .map(|v| bit & 1 << v != 0)
                            .collect::<Vec<_>>()
                    })
                    .filter(|vec| vec.iter().filter(|v| **v).count() == 12)
                    .map(|vec| {
                        vec.iter()
                            .enumerate()
                            .filter(|v| *v.1)
                            .enumerate()
                            .map(|v| numbers[v.1 .0] * 10u128.pow(11 - v.0 as u32))
                            .sum::<u128>()
                    })
                    .max()
                    .unwrap()
            })
            .sum::<u128>()
    );*/
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|line| {
                let mut numbers = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or_default() as u128)
                    .enumerate()
                    .collect::<HashSet<_>>();
                let numbers_len = numbers.len();
                let mut ans = Vec::<(usize, u128)>::with_capacity(12);
                for ind in (0..12).rev() {
                    let mut temp = Vec::new();
                    loop {
                        let maxi = numbers
                            .iter()
                            .max_by_key(|v| (v.1, -(v.0 as isize)))
                            .unwrap()
                            .clone();
                        numbers.remove(&maxi);
                        // some checks
                        if maxi.0 + ind >= numbers_len {
                            temp.push(maxi);
                            continue;
                        }
                        if let Some(last) = ans.last()
                            && maxi.0 < last.0
                        {
                            temp.push(maxi);
                            continue;
                        }
                        ans.push(maxi);
                        break;
                    }
                    temp.into_iter().for_each(|v| {
                        numbers.insert(v);
                    });
                }
                ans.iter()
                    .enumerate()
                    .map(|v| v.1.1 * 10u128.pow(11u32 - v.0 as u32))
                    .sum::<u128>()
            })
            .sum::<u128>()
    );
}
