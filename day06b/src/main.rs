pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .windows(14)
            .position(|b| !(0..13).any(|i| (i + 1..14).any(|j| b[i] == b[j])))
            .unwrap()
            + 14,
    );
}
