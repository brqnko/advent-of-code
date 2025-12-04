use std::io::BufRead;

enum Mass {
    Roll,
    None,
}

impl From<char> for Mass {
    fn from(value: char) -> Self {
        match value {
            '@' => Mass::Roll,
            _ => Mass::None,
        }
    }
}

fn main() {
    const DIRECTIONS: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut grid = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Mass>>())
        .collect::<Vec<_>>();
    // part 1
    // println!(
    //     "{}",
    //     (0..grid.len() as i32)
    //         .map(|i| (0..grid[i as usize].len() as i32).map(move |j| (i, j)))
    //         .flatten()
    //         .filter(|(i, j)| matches!(grid[*i as usize][*j as usize], Mass::Roll))
    //         .filter(|(i, j)| DIRECTIONS
    //             .iter()
    //             .map(|(di, dj)| (i + di, j + dj))
    //             .filter(|(i, j)| (0..grid.len() as i32).contains(i)
    //                 && (0..grid[*i as usize].len() as i32).contains(j))
    //             .filter(|(i, j)| matches!(grid[*i as usize][*j as usize], Mass::Roll))
    //             .count()
    //             < 4)
    //         .count()
    // );

    // part 2
    let mut ans = 0;
    loop {
        let mut flag = false;
        for (i, j) in (0..grid.len() as i32)
            .map(|i| (0..grid[i as usize].len() as i32).map(move |j| (i, j)))
            .flatten()
            .filter(|(i, j)| matches!(grid[*i as usize][*j as usize], Mass::Roll))
            .filter(|(i, j)| {
                DIRECTIONS
                    .iter()
                    .map(|(di, dj)| (i + di, j + dj))
                    .filter(|(i, j)| {
                        (0..grid.len() as i32).contains(i)
                            && (0..grid[*i as usize].len() as i32).contains(j)
                    })
                    .filter(|(i, j)| matches!(grid[*i as usize][*j as usize], Mass::Roll))
                    .count()
                    < 4
            })
            .collect::<Vec<_>>()
        {
            grid[i as usize][j as usize] = Mass::None;
            flag = true;
            ans += 1;
        }

        if !flag {
            break;
        }
    }
    println!("{}", ans,);
}
