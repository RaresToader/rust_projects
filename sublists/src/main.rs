#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if is_superlist(a, b) {
        Comparison::Superlist
    } else if is_superlist(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|w| w == b)
}

fn main() {
    println!("Hello, world!");
}
