/// # 9. Palindrome Number
///
/// Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
///
/// # Example 1:
///
/// Input: 121
/// Output: true
/// # Example 2:
///
/// Input: -121
/// Output: false
/// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
/// # Example 3:
///
/// Input: 10
/// Output: false
/// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
/// Follow up:
///
/// Coud you solve it without converting the integer to a string?
///
pub trait PalindromeNumber {
    fn is_palindrome(x: i32) -> bool;
}

pub struct Solution1;

impl PalindromeNumber for Solution1 {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x > 0 && x % 10 == 0) {
            return false;
        }

        let mut v = Vec::new();
        let mut t = x;

        while t > 0 {
            v.push(t % 10);
            t = t / 10;
        }

        let len = v.len();
        for i in 0..len / 2 {
            if v[i] != v[len - 1 - i] {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::is_palindrome(121), true);
    assert_eq!(Solution1::is_palindrome(-121), false);
    assert_eq!(Solution1::is_palindrome(10), false);
    assert_eq!(Solution1::is_palindrome(13231), true);
    assert_eq!(Solution1::is_palindrome(127898721), true);
    assert_eq!(Solution1::is_palindrome(1000000), false);
}

pub struct Solution2;

impl PalindromeNumber for Solution2 {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x > 0 && x % 10 == 0) {
            return false;
        }

        let mut reverted = 0;
        let mut v = x;
        while reverted < v {
            reverted = reverted * 10 + v % 10;
            v /= 10;
        }

        reverted == v || reverted / 10 == v
    }
}

#[test]
fn test_solution2() {
    assert_eq!(Solution2::is_palindrome(121), true);
    assert_eq!(Solution2::is_palindrome(-121), false);
    assert_eq!(Solution2::is_palindrome(10), false);
    assert_eq!(Solution2::is_palindrome(13231), true);
    assert_eq!(Solution2::is_palindrome(127898721), true);
    assert_eq!(Solution2::is_palindrome(1000000), false);
}