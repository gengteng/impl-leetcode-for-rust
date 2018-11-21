/// # 5. Longest Palindromic Substring
///
/// Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
///
/// # Example 1:
///
/// Input: "babad"
/// Output: "bab"
/// Note: "aba" is also a valid answer.
/// # Example 2:
///
/// Input: "cbbd"
/// Output: "bb"
pub trait LongestPalindromicSubstring {
    fn longest_palindrome(s: &str) -> &str;
}

pub struct Solution1;

impl Solution1 {
    fn expand_around_center(s: &str, left: usize, right: usize) -> usize {
        let mut l = left;
        let mut r = right;
        let b = s.as_bytes();
        let len = b.len();

        while r < len && b[l] == b[r] {
            if l == 0 {
                return r + 1;
            }

            l -= 1;
            r += 1;
        }

        r - l - 1
    }
}

impl LongestPalindromicSubstring for Solution1 {
    fn longest_palindrome(s: &str) -> &str {
        use std::cmp::max;

        let mut start = 0;
        let mut end = 0;

        for i in 0..s.len() {
            let len1 = Self::expand_around_center(s, i, i);
            let len2 = Self::expand_around_center(s, i, i + 1);
            let len = max(len1, len2);

            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }

        &s[start..end + 1]
    }
}


#[test]
fn test_solution1() {
    assert_eq!(Solution1::longest_palindrome("cbbd"), "bb");
    assert_eq!(Solution1::longest_palindrome("abcdedcbf"), "bcdedcb");
}
