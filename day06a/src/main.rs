pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .windows(4)
            .position(|b| !(0..4).any(|i| (i + 1..4).any(|j| b[i] == b[j])))
            .unwrap()
            + 4,
    );
}
