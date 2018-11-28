/// # 15. 3Sum
///
/// Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
///
/// # Note:
///
/// The solution set must not contain duplicate triplets.
///
/// # Example:
///
/// Given array nums = [-1, 0, 1, 2, -1, -4],
///
/// A solution set is:
/// [
/// [-1, 0, 1],
/// [-1, -1, 2]
/// ]
///
use std::collections::HashSet;

pub trait ThreeSum {
    fn three_sum(nums: &[i32]) -> HashSet<[i32;3]>;
}

pub struct Solution1;

impl ThreeSum for Solution1 {
    fn three_sum(nums: &[i32]) -> HashSet<[i32;3]> {
        let mut result = HashSet::new();

        for (i, a) in nums.iter().enumerate() {
            for (j, b) in nums.iter().skip(i + 1).enumerate() {
                for c in nums.iter().skip(i + j + 2) {
                    if a + b + c == 0 {
                        let mut v = [*a, *b, *c];
                        v.sort();
                        result.insert(v);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::ThreeSum;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        use std::collections::HashSet;

        assert_eq!(Solution1::three_sum(&[-1, 0, 1, 2, -1, -4]), {
            let mut result = HashSet::new();
            result.insert([-1, 0, 1]);
            result.insert([-1, -1, 2]);
            result
        });
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::three_sum(&[-1, 0, 1, 2, -1, -4]));
    }
}