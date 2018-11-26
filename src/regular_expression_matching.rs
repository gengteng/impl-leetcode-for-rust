/// # 10. Regular Expression Matching
///
/// Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
///
/// '.' Matches any single character.
/// '*' Matches zero or more of the preceding element.
/// The matching should cover the entire input string (not partial).
///
/// # Note:
///
/// s could be empty and contains only lowercase letters a-z.
/// p could be empty and contains only lowercase letters a-z, and characters like . or *.
/// # Example 1:
///
/// Input:
/// s = "aa"
/// p = "a"
/// Output: false
/// Explanation: "a" does not match the entire string "aa".
/// # Example 2:
///
/// Input:
/// s = "aa"
/// p = "a*"
/// Output: true
/// Explanation: '*' means zero or more of the precedeng element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
/// # Example 3:
///
/// Input:
/// s = "ab"
/// p = ".*"
/// Output: true
/// Explanation: ".*" means "zero or more (*) of any character (.)".
/// # Example 4:
///
/// Input:
/// s = "aab"
/// p = "c*a*b"
/// Output: true
/// Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".
/// # Example 5:
///
/// Input:
/// s = "mississippi"
/// p = "mis*is*p*."
/// Output: false
///
pub trait RegularExpressionMatching {
    fn is_match(s: &str, p: &str) -> bool;
}

pub struct Solution1;

impl RegularExpressionMatching for Solution1 {
    fn is_match(s: &str, p: &str) -> bool {
        use regex::Regex;

        match Regex::new(["^", p, "$"].join("").as_str()) {
            Ok(r) => {
                r.is_match(s)
            },
            _ => {
                false
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::RegularExpressionMatching;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::is_match("aa", "a"), false);
        assert_eq!(Solution1::is_match("aa", "a*"), true);
        assert_eq!(Solution1::is_match("ab", ".*"), true);
        assert_eq!(Solution1::is_match("aab", "c*a*b"), true);
        assert_eq!(Solution1::is_match("mississippi", "mis*is*p*."), false);
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::is_match("aab", "c*a*b"));
        b.iter(|| Solution1::is_match("mississippi", "mis*is*p*."));
    }
}