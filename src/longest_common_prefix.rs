/// # 14. Longest Common Prefix
///
/// Write a function to find the longest common prefix string amongst an array of strings.
///
/// If there is no common prefix, return an empty string "".
///
/// Example 1:
///
/// Input: ["flower","flow","flight"]
/// Output: "fl"
/// Example 2:
///
/// Input: ["dog","racecar","car"]
/// Output: ""
/// Explanation: There is no common prefix among the input strings.
/// Note:
///
/// All given inputs are in lowercase letters a-z.
pub trait LongestCommonPrefix {
    fn longest_common_prefix(strings: &[&str]) -> String;
}

pub struct Solution1;

impl LongestCommonPrefix for Solution1 {
    fn longest_common_prefix(strings: &[&str]) -> String {
        let mut result = String::new();

        if let Some(f) = strings.iter().nth(0) {
            for (i, c) in f.chars().enumerate() {
                for s in strings.iter().skip(1) {
                    if let Some(v) = s.chars().nth(i) {
                        if v != c {
                            return result;
                        }
                    } else {
                        return result;
                    }
                }

                result.push(c);
            }
        }

        result
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::longest_common_prefix(&["flower","flow","flight"]), "fl");
    assert_eq!(Solution1::longest_common_prefix(&["dog","race_car","car"]), "");
    assert_eq!(Solution1::longest_common_prefix(&["same","same","same"]), "same");
    assert_eq!(Solution1::longest_common_prefix(&["test"]), "test");
}