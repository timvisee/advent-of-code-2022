pub fn main() {
    let mut cals = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    cals.sort_unstable();
    println!("{}", cals.into_iter().rev().take(3).sum::<u32>());
}
