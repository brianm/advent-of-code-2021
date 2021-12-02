use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let nums: Vec<u32> = include_str!("input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    let one = nums.iter().tuple_windows().filter(|(a, b)| b > a).count();
    println!("{}", one);

    // 1 2 3
    //   2 3 4
    //     3 4 5
    // so 3tuple - next 3tuple is t[3] - t[0] for each 4tuple
    // 1 2 3 4
    //   2 3 4 5
    //     3 4 5 6
    let three = nums.windows(4).filter(|w| w[3] > w[0]).count();
    println!("{}", three);

    let two2 = nums
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(a, _, _, b)| b > a)
        .count();
    println!("{}", two2);

    Ok(())
}
