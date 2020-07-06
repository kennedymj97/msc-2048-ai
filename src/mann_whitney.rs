use statrs::distribution::{Normal, Univariate};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd)]
struct Sample<S, T>
where
    S: Iterator<Item = T>,
    T: Ord,
{
    data: S,
}

impl<S, T> Ord for Sample<S, T>
where
    S: Iterator<Item = T>,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        mann_whitney_u_test(self, other)
    }
}

enum Group {
    X,
    Y,
}

fn mann_whitney_u_test<S, T>(xs: S, ys: S) -> Ordering
where
    S: Iterator<Item = T>,
    T: Ord,
{
    let mut vs = xs
        .map(|x| (x, Group::X))
        .chain(ys.map(|y| (y, Group::Y)))
        .collect::<Vec<_>>();
    vs.sort();

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_normal_cdf() {
        let n = Normal::new(0., 1.).unwrap();
        assert!((n.cdf(2.35) - 0.99061).abs() < 0.000005);
        assert!((n.cdf(-3.9) - 0.00005).abs() < 0.000005);
    }
}
