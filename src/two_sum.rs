/// 1. Two Sum
///
/// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// Example:
/// ```
/// Given nums = [2, 7, 11, 15], target = 9,
///
/// Because nums[0] + nums[1] = 2 + 7 = 9,
/// return [0, 1].
/// ```
pub trait TwoSum {
    fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>;
}

pub struct Solution1;

impl TwoSum for Solution1 {
    fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        let len = nums.len();
        for i in 0..len {
            for j in 1..len {
                if (nums[i] == target - nums[j]) && i != j {
                    return Some((i, j));
                }
            }
        }

        None
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::two_sum(&[2, 3, 7, 11], 9), Some((0, 2)));
}

pub struct Solution2;

impl TwoSum for Solution2 {
    fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut map = ::std::collections::HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            map.insert(v, i);
        }

        for (i, v) in nums.iter().enumerate() {
            let k = target - v;
            match map.get(&k) {
                Some(j) if i != *j => {
                    return Some((i, *j));
                },
                _ => {

                }
            }
        }
        None
    }
}

#[test]
fn test_solution2() {
    assert_eq!(Solution2::two_sum(&[2, 3, 7, 11], 9), Some((0, 2)));
}

pub struct Solution3;

impl TwoSum for Solution3 {
    fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut map = ::std::collections::HashMap::<i32, usize>::new();
        for (i, &v) in nums.iter().enumerate() {
            let k = target - v;
            match map.get(&k) {
                Some(&j) if j != i => {
                    return Some(if j > i {(i, j)} else {(j, i)});
                },
                _ => {

                }
            }
            map.insert(v, i);
        }
        None
    }
}

#[test]
fn test_solution3() {
    assert_eq!(Solution3::two_sum(&[2, 3, 7, 11], 9), Some((0, 2)));
}
