pub fn main() {
    let d = include_bytes!("../input.txt");

    let mut w = 0;
    'main: loop {
        for i in (0..13).rev() {
            for j in (i + 1..14).rev() {
                if d[w + i] == d[w + j] {
                    w += i + 1;
                    continue 'main;
                }
            }
        }
        break;
    }

    println!("{}", w + 14);
}
