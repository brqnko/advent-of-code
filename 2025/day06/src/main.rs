use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // part 1
    // let r = std::io::stdin()
    //     .lock()
    //     .lines()
    //     .filter_map(Result::ok)
    //     .map(|line| {
    //         line.split(" ")
    //             .filter(|s| s != &" " && s.len() != 0)
    //             .map(ToOwned::to_owned)
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();

    // let r_nums = r[0..r.len() - 1]
    //     .iter()
    //     .map(|r| {
    //         r.iter()
    //             .map(|e| e.parse::<i128>().unwrap())
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();

    // let answers = r
    //     .last()
    //     .unwrap()
    //     .iter()
    //     .enumerate()
    //     .map(|(ind, op)| match op.as_str() {
    //         "*" => r_nums
    //             .iter()
    //             .map(|r| r[ind])
    //             .fold(1i128, |current, item| current * item),
    //         "+" => r_nums.iter().map(|r| r[ind]).sum::<i128>(),
    //         _ => panic!("unknown operator"),
    //     })
    //     .collect::<Vec<_>>();

    // println!("{}", answers.iter().sum::<i128>());

    // part 2
    let r = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // extract operators from last line
    let last_line = format!("{} \n", r.last().unwrap());
    let mut operators = Vec::new();
    let mut operator_ind = None;
    for (ind, c) in last_line.chars().enumerate() {
        match c {
            '+' | '*' | '\n' => {
                if let Some(operator_ind) = operator_ind {
                    operators.push((
                        &last_line[operator_ind..operator_ind + 1],
                        operator_ind,
                        ind,
                    ));
                }
                operator_ind = Some(ind);
            }
            _ => (),
        }
    }

    // println!("{:?}", operators);

    // calc answers for each line
    let answers = operators.into_iter().map(|(op, start, end)| {
        let strs = r[0..r.len() - 1].iter().map(|line| &line[start..end - 1]).collect::<Vec<_>>();
        (
            op,
            (0..strs[0].len()).map(|i| strs.iter().map(move |s| &s[i..i + 1]).filter(|c| c != &" ").collect::<String>().parse::<i128>().unwrap()).collect::<Vec<_>>(),
        )
    }).map(|(op, numbers)| match op {
        "*" => numbers.iter().fold(1, |current, item| current * item),
        "+" => numbers.iter().sum::<i128>(),
        _ => panic!("unknown operator"),
    }).collect::<Vec<_>>();
    println!("{}", answers.iter().sum::<i128>());
    Ok(())
}
