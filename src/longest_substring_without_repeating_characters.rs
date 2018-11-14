/// 3. Longest Substring Without Repeating Characters
///
/// Given a string, find the length of the longest substring without repeating characters.
///
/// Examples:
/// ```
/// Given "abcabcbb", the answer is "abc", which the length is 3.
///
/// Given "bbbbb", the answer is "b", with the length of 1.
///
/// Given "pwwkew", the answer is "wke", with the length of 3. Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
/// ```
pub trait LongestSubstringWithoutRepeatingCharacters {
    fn longest_substring_without_repeating_characters(s: &str) -> u32;
}

pub struct Solution1;

impl LongestSubstringWithoutRepeatingCharacters for Solution1 {
    fn longest_substring_without_repeating_characters(s: &str) -> u32 {
        let mut max: u32 = 0;
        for (i, a) in s.as_bytes().iter().enumerate() {
            let mut set = ::std::collections::HashSet::new();
            set.insert(a);

            for b in s.as_bytes().iter().skip(i+1) {
                if set.contains(b) {
                    break;
                }

                set.insert(b);
            }

            let len = set.len() as u32;
            if len > max {
                max = len;
            }
        }

        max
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::longest_substring_without_repeating_characters("abcabcbb"), 3);
    assert_eq!(Solution1::longest_substring_without_repeating_characters("bbbbbbb"), 1);
    assert_eq!(Solution1::longest_substring_without_repeating_characters("pwwkew"), 3);
}