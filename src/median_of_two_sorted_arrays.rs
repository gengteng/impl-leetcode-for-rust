/// # 4. Median of Two Sorted Arrays
///
/// There are two sorted arrays nums1 and nums2 of size m and n respectively.
///
/// Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
///
/// You may assume nums1 and nums2 cannot be both empty.
///
///
///
/// Example 1:
///
/// nums1 = [1, 3]
/// nums2 = [2]
///
/// The median is 2.0
/// Example 2:
///
/// nums1 = [1, 2]
/// nums2 = [3, 4]
///
/// The median is (2 + 3)/2 = 2.5
pub trait MedianOfTwoSortedArrays {
    fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64;
}

pub struct Solution1;

impl MedianOfTwoSortedArrays for Solution1 {
    fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
        use std::cmp::{max, min};

        let mut m = nums1.len();
        let mut n = nums2.len();
        let mut a = nums1;
        let mut b = nums2;

        if m > n {
            let t = m;
            m = n;
            n = t;

            a = nums2;
            b = nums1;
        }

        let mut i_min = 0;
        let mut i_max = m;
        let half_len = (m + n + 1) / 2;

        while i_min <= i_max {
            let i = (i_min + i_max) / 2;
            let j = half_len - i;
            if i < i_max && b[j - 1] > a[i] {
                i_min = i + 1; // i is too small
            } else if i > i_min && a[i - 1] > b[j] {
                i_max = i - 1; // i is too big
            } else { // i is perfect
                let max_left = if i == 0 { b[j - 1] } else if j == 0 { a[i - 1] } else { max(a[i - 1], b[j - 1]) };
                if (m + n) % 2 == 1 {
                    return max_left as f64;
                }

                let min_right = if i == m { b[j] } else if j == n { a[i] } else { min(b[j], a[i]) };
                return (max_left + min_right) as f64 / 2.0;
            }
        }

        0.0
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::find_median_sorted_arrays(&[1, 2], &[3, 4]), 2.5);
    assert_eq!(Solution1::find_median_sorted_arrays(&[1, 3], &[2]), 2.0);
    assert_eq!(Solution1::find_median_sorted_arrays(&[4,6,8], &[1,3,5]), 4.5);
}