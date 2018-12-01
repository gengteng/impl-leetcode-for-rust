/// # 16. 3Sum Closest
///
/// Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
///
/// # Example:
///
/// Given array nums = [-1, 2, 1, -4], and target = 1.
///
/// The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
pub trait ThreeSumClosest {
    fn three_sum_closest(nums: &[i32], target: i32) -> i32;
}

pub struct Solution1;

impl ThreeSumClosest for Solution1 {
    fn three_sum_closest(nums: &[i32], target: i32) -> i32 {
        let mut min_abs = std::i32::MAX;
        let mut result = 0;

        for (i, a) in nums.iter().enumerate() {
            for (j, b) in nums.iter().skip(i + 1).enumerate() {
                for c in nums.iter().skip(i + j + 2) {
                    let abs = (a + b + c - target).abs();
                    if abs < min_abs {
                        result = a + b + c;
                        min_abs = abs;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::ThreeSumClosest;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::three_sum_closest(&[-1, 2, -4, 1], 1), 2);
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::three_sum_closest(&[-1, 2, 1, -4], 1));
    }
}