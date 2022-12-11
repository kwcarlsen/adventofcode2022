fn divisible_by(i: i32, j: i32) -> bool {
    i % j
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{Direction, Position};

    quickcheck! {
        fn prop_reverse_reverse(xs: Vec<usize>) -> bool {
            let rev: Vec<_> = xs.clone().into_iter().rev().collect();
            let revrev: Vec<_> = rev.into_iter().rev().collect();
            xs == revrev
        }
    }
}