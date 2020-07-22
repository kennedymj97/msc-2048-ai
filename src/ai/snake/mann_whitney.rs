use statrs::distribution::{Normal, Univariate};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Group {
    X,
    Y,
}

pub fn mann_whitney_u_test_01<T: PartialOrd>(xs: Vec<T>, ys: Vec<T>) -> Ordering {
    let mann_whitney_u_test = MannWhitneyUTest::new(xs, ys);
    assert!(mann_whitney_u_test.nx > 0. && mann_whitney_u_test.ny > 0.);
    mann_whitney_u_test.test_01()
}

pub fn mann_whitney_u_test_05<T: PartialOrd>(xs: Vec<T>, ys: Vec<T>) -> Ordering {
    let mann_whitney_u_test = MannWhitneyUTest::new(xs, ys);
    assert!(mann_whitney_u_test.nx > 0. && mann_whitney_u_test.ny > 0.);
    mann_whitney_u_test.test_05()
}

struct MannWhitneyUTest {
    counts: Vec<(f64, f64)>,
    nx: f64,
    ny: f64,
}

impl MannWhitneyUTest {
    fn new<T: PartialOrd>(xs: Vec<T>, ys: Vec<T>) -> Self {
        let mut vs = xs
            .iter()
            .map(|x| (x, Group::X))
            .chain(ys.iter().map(|y| (y, Group::Y)))
            .collect::<Vec<_>>();
        vs.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());

        let mut prev = None;
        let mut counts = Vec::new();
        let mut nx = 0.;
        let mut ny = 0.;
        vs.iter().for_each(|(v, group)| {
            if prev != Some(v) {
                counts.push((0., 0.));
            }
            match group {
                Group::X => {
                    counts.last_mut().unwrap_or_else(|| unreachable!()).0 += 1.;
                    nx += 1.;
                }
                Group::Y => {
                    counts.last_mut().unwrap_or_else(|| unreachable!()).1 += 1.;
                    ny += 1.;
                }
            }
            prev = Some(v);
        });
        MannWhitneyUTest { counts, nx, ny }
    }

    fn test_05(&self) -> Ordering {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let (ux, uy) = self.uxy();
        let u = ux.min(uy);

        if nx <= 20 && ny <= 20 {
            let u_thresh = MANN_WHITNEY_TABLE_P05[nx - 1][ny - 1];
            if u < u_thresh as f64 {
                match ux < uy {
                    true => return Ordering::Less,
                    false => return Ordering::Greater,
                }
            } else {
                return Ordering::Equal;
            }
        }

        if self.p() < 0.05 {
            match ux < uy {
                true => Ordering::Less,
                false => Ordering::Greater,
            }
        } else {
            Ordering::Equal
        }
    }

    fn test_01(&self) -> Ordering {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let (ux, uy) = self.uxy();
        let u = ux.min(uy);

        if nx <= 20 && ny <= 20 {
            let u_thresh = MANN_WHITNEY_TABLE_P01[nx - 1][ny - 1];
            if u < u_thresh as f64 {
                match ux < uy {
                    true => return Ordering::Less,
                    false => return Ordering::Greater,
                }
            } else {
                return Ordering::Equal;
            }
        }

        if self.p() < 0.01 {
            match ux < uy {
                true => Ordering::Less,
                false => Ordering::Greater,
            }
        } else {
            Ordering::Equal
        }
    }
    fn uxy(&self) -> (f64, f64) {
        let mut rank = 1.;
        let rx = self.counts.iter().fold(0., |mut acc, &(x_count, y_count)| {
            let new_rank = rank + x_count + y_count;
            if x_count + y_count == 1. {
                acc += rank * x_count;
            } else {
                acc += x_count * ((rank + (new_rank - 1.)) / 2.);
            }
            rank = new_rank;
            acc
        });
        let nx = self.nx;
        let ny = self.ny;
        let ux: f64 = rx - ((nx * (nx + 1.)) / 2.);
        let uy: f64 = (nx * ny) - ux;
        (ux, uy)
    }

    fn u(&self) -> f64 {
        let (ux, uy) = self.uxy();
        ux.min(uy)
    }

    fn mu(&self) -> f64 {
        (self.nx * self.ny) / 2.
    }

    fn sigmau(&self) -> f64 {
        let t = self
            .counts
            .iter()
            .map(|&(x, y)| x + y)
            .map(|t| t * t * t - t)
            .sum::<f64>() as f64;
        let nx = self.nx;
        let ny = self.ny;
        let n = nx + ny;
        ((nx * ny * ((n + 1.0) - t / (n * (n - 1.0)))) / 12.0).sqrt()
    }

    fn z(&self) -> f64 {
        (self.u() - self.mu()) / self.sigmau()
    }

    fn p(&self) -> f64 {
        let standard_normal = Normal::new(0., 1.).unwrap();
        (1. - standard_normal.cdf(self.z().abs())) * 2.
    }
}

const MANN_WHITNEY_TABLE_P05: [[i8; 20]; 20] = [
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    [
        -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2,
    ],
    [
        -1, -1, -1, -1, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8,
    ],
    [
        -1, -1, -1, 0, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 11, 11, 12, 13, 13,
    ],
    [
        -1, -1, 0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 17, 18, 19, 20,
    ],
    [
        -1, -1, 1, 2, 3, 5, 6, 8, 10, 11, 13, 14, 16, 17, 19, 21, 22, 24, 25, 27,
    ],
    [
        -1, -1, 1, 3, 5, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34,
    ],
    [
        -1, 0, 2, 4, 6, 8, 10, 13, 15, 17, 19, 22, 24, 26, 29, 31, 34, 36, 38, 41,
    ],
    [
        -1, 0, 2, 4, 7, 10, 12, 15, 17, 21, 23, 26, 28, 31, 34, 37, 39, 42, 45, 48,
    ],
    [
        -1, 0, 3, 5, 8, 11, 14, 17, 20, 23, 26, 29, 33, 36, 39, 42, 45, 48, 52, 55,
    ],
    [
        -1, 0, 3, 6, 9, 13, 16, 19, 23, 26, 30, 33, 37, 40, 44, 47, 51, 55, 58, 62,
    ],
    [
        -1, 1, 4, 7, 11, 14, 18, 22, 26, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65, 69,
    ],
    [
        -1, 1, 4, 8, 12, 16, 20, 24, 28, 33, 37, 41, 45, 50, 54, 59, 63, 67, 72, 76,
    ],
    [
        -1, 1, 5, 9, 13, 17, 22, 26, 31, 36, 40, 45, 50, 55, 59, 64, 67, 74, 78, 83,
    ],
    [
        -1, 1, 5, 10, 14, 19, 24, 29, 34, 39, 44, 49, 54, 59, 64, 70, 75, 80, 85, 90,
    ],
    [
        -1, 1, 6, 11, 15, 21, 26, 31, 37, 42, 47, 53, 59, 64, 70, 75, 81, 86, 92, 98,
    ],
    [
        -1, 2, 6, 11, 17, 22, 28, 34, 39, 45, 51, 57, 63, 67, 75, 81, 87, 93, 99, 105,
    ],
    [
        -1, 2, 7, 12, 18, 24, 30, 36, 42, 48, 55, 61, 67, 74, 80, 86, 93, 99, 106, 112,
    ],
    [
        -1, 2, 7, 13, 19, 25, 32, 38, 45, 52, 58, 65, 72, 78, 85, 92, 99, 106, 113, 119,
    ],
    [
        -1, 2, 8, 14, 20, 27, 34, 41, 48, 55, 62, 69, 76, 83, 90, 98, 105, 112, 119, 127,
    ],
];

const MANN_WHITNEY_TABLE_P01: [[i8; 20]; 20] = [
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0,
    ],
    [
        -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3,
    ],
    [
        -1, -1, -1, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8,
    ],
    [
        -1, -1, -1, -1, 0, 1, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 11, 12, 13,
    ],
    [
        -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 15, 16, 17, 18,
    ],
    [
        -1, -1, -1, 0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24,
    ],
    [
        -1, -1, -1, 1, 2, 4, 6, 7, 9, 11, 13, 15, 17, 18, 20, 22, 24, 26, 28, 30,
    ],
    [
        -1, -1, 0, 1, 3, 5, 7, 9, 11, 13, 16, 18, 20, 22, 24, 27, 29, 31, 33, 36,
    ],
    [
        -1, -1, 0, 2, 4, 6, 9, 11, 13, 16, 18, 21, 24, 26, 29, 31, 34, 37, 39, 42,
    ],
    [
        -1, -1, 0, 2, 5, 7, 10, 13, 16, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 46,
    ],
    [
        -1, -1, 1, 3, 6, 9, 12, 15, 18, 21, 24, 27, 31, 34, 37, 41, 44, 47, 51, 54,
    ],
    [
        -1, -1, 1, 3, 7, 10, 13, 17, 20, 24, 27, 31, 34, 38, 42, 45, 49, 53, 56, 60,
    ],
    [
        -1, -1, 1, 4, 7, 11, 15, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 63, 67,
    ],
    [
        -1, -1, 2, 5, 8, 12, 16, 20, 24, 29, 33, 37, 42, 46, 51, 55, 60, 64, 69, 73,
    ],
    [
        -1, -1, 2, 5, 9, 13, 18, 22, 27, 31, 36, 41, 45, 50, 55, 60, 65, 70, 74, 79,
    ],
    [
        -1, -1, 2, 6, 10, 15, 19, 24, 29, 34, 39, 44, 49, 54, 60, 65, 70, 75, 81, 86,
    ],
    [
        -1, -1, 2, 6, 11, 16, 21, 26, 31, 37, 42, 47, 53, 58, 64, 70, 75, 81, 87, 92,
    ],
    [
        -1, 0, 3, 7, 12, 17, 22, 28, 33, 39, 45, 51, 56, 63, 69, 74, 81, 87, 93, 99,
    ],
    [
        -1, 0, 3, 8, 13, 18, 24, 30, 36, 42, 46, 54, 60, 67, 73, 79, 86, 92, 99, 105,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_normal_cdf() {
        let n = Normal::new(0., 1.).unwrap();
        assert!((n.cdf(2.35) - 0.99061).abs() < 0.000005);
        assert!((n.cdf(-3.9) - 0.00005).abs() < 0.000005);
    }

    #[test]
    fn it_mann_whitney_u_test() {
        let control = vec![11, 15, 9, 4, 34, 17, 18, 14, 12, 13, 26, 31];
        let drug = vec![31, 34, 35, 29, 28, 12, 18, 30, 14, 22, 10];
        let mann_whitney_u_test = MannWhitneyUTest::new(control, drug);
        assert_eq!(mann_whitney_u_test.u(), 39.5);
        assert_eq!(mann_whitney_u_test.test_05(), Ordering::Equal);

        let non_smokers: Vec<f64> = vec![
            58.5, 9., 71., 54.5, 15., 61.5, 66.5, 37., 68.5, 68.5, 42., 75., 29., 44., 66.5, 49.5,
            64., 46.5, 71., 3.5, 6.5, 24.5, 64., 44., 76., 24.5, 77., 29., 78., 6.5, 17.5, 73.5,
            58.5, 37., 37., 40.5, 29., 49.5, 60., 20.5,
        ];

        let smokers: Vec<f64> = vec![
            37., 61.5, 49.5, 17.5, 56.5, 2., 64., 54.5, 11., 52.5, 49.5, 20.5, 20.5, 37., 15., 15.,
            24.5, 44., 71., 46.5, 9., 40.5, 5., 29., 20.5, 33., 24.5, 12.5, 9., 52.5, 73.5, 12.5,
            33., 56.5, 1., 29., 3.5, 33.,
        ];
        let mann_whitney_u_test = MannWhitneyUTest::new(non_smokers, smokers);
        assert_eq!(mann_whitney_u_test.uxy(), (1034., 486.));
        assert_eq!(mann_whitney_u_test.u(), 486.);
        assert_eq!(mann_whitney_u_test.mu(), 760.);
        assert!((mann_whitney_u_test.sigmau() - 100.0333278) < 0.00000005);
        assert!((mann_whitney_u_test.z() - 2.739087123) < 0.0000000005);
        assert!((mann_whitney_u_test.p() - 0.006161) < 0.0000005);
        assert_eq!(mann_whitney_u_test.test(), Ordering::Greater);
    }
}
