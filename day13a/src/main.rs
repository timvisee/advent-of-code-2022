use nom::*;
use std::cmp::Ordering;

pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|p| pair(p.as_bytes()).unwrap().1)
            .enumerate()
            .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum::<usize>(),
    );
}

#[derive(PartialEq, Eq)]
enum Item {
    I(u8),
    L(Vec<Item>),
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::I(a), Item::I(b)) => a.cmp(b),
            (Item::L(a), Item::L(b)) => match a.iter().cmp(b) {
                r if r != Ordering::Equal => r,
                _ => a.len().cmp(&b.len()),
            },
            (Item::I(_), Item::L(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Item::I(a), Item::L(_)) => Item::L(vec![Item::I(*a)]).cmp(other),
            (Item::L(_), Item::I(_)) => other.cmp(self).reverse(),
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

named!(item<&[u8], Item>, alt!(map!(list, Item::L) | map!(num, Item::I)));
named!(num<&[u8], u8>, map_opt!(nom::character::complete::digit1, atoi::atoi));
named!(list<&[u8], Vec<Item>>, delimited!(char!('['), separated_list0!(char!(','), item), char!(']')));
named!(pair<&[u8], (Item, Item)>, separated_pair!(item, tag!("\n"), item));
