pub fn main() {
    let mut i = 0;
    while let Some(win) = include_bytes!("../input.txt").get(i..i + 14) {
        let mut seen = 0u32;
        if let Some(pos) = win.iter().rposition(|b| {
            let bit = 1 << (b % 32);
            let duplicate = seen & bit != 0;
            seen |= bit;
            duplicate
        }) {
            i += pos + 1;
        } else {
            break;
        }
    }

    println!("{}", i);
}
