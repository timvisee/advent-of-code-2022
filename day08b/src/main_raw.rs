// Implementation querying raw input, doesn't seem to be faster.

pub fn main() {
    let d = include_bytes!("../input.txt");
    let s = d.iter().position(|b| b == &b'\n').unwrap();
    println!(
        "{}",
        (0..s * s)
            .map(|i| {
                let icur = i + i / s;
                let c = d[icur];
                if c <= b'1' {
                    return 0;
                }

                ((1..(i / s))
                    .map(|ii| d[icur - ii * (s + 1)])
                    .position(|h| h >= c)
                    .unwrap_or_else(|| (i / s).wrapping_sub(1))
                    .wrapping_add(1))
                    * ((1..s - (i / s))
                        .map(|ii| d[icur + ii * (s + 1)])
                        .position(|h| h >= c)
                        .unwrap_or_else(|| (s - (i / s)).wrapping_sub(2))
                        .wrapping_add(1))
                    * ((1..(i % s))
                        .map(|ii| d[icur - ii])
                        .position(|h| h >= c)
                        .unwrap_or_else(|| (i % s).wrapping_sub(1))
                        .wrapping_add(1))
                    * ((1..s - (i % s))
                        .map(|ii| d[icur + ii])
                        .position(|h| h >= c)
                        .unwrap_or_else(|| (s - (i % s)).wrapping_sub(2))
                        .wrapping_add(1))
            })
            .max()
            .unwrap()
    );
}
