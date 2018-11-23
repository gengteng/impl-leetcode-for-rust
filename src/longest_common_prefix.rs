/// # 14. Longest Common Prefix
///
/// Write a function to find the longest common prefix string amongst an array of strings.
///
/// If there is no common prefix, return an empty string "".
///
/// # Example 1:
///
/// Input: ["flower","flow","flight"]
/// Output: "fl"
/// # Example 2:
///
/// Input: ["dog","racecar","car"]
/// Output: ""
/// Explanation: There is no common prefix among the input strings.
/// # Note:
///
/// All given inputs are in lowercase letters a-z.
pub trait LongestCommonPrefix {
    fn longest_common_prefix(strings: &[&str]) -> String;
}

pub struct Solution1;

impl LongestCommonPrefix for Solution1 {
    fn longest_common_prefix(strings: &[&str]) -> String {
        let mut len = 0;

        if let Some(f) = strings.iter().nth(0) {
            for (i, c) in f.chars().enumerate() {
                for s in strings.iter().skip(1) {
                    if let Some(v) = s.chars().nth(i) {
                        if v != c {
                            return f[0..len].to_string();
                        }
                    } else {
                        return f[0..len].to_string();
                    }
                }

                len += 1;
            }

            f[0..len].to_string()
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::LongestCommonPrefix;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::longest_common_prefix(&["flower","flow","flight"]), "fl");
        assert_eq!(Solution1::longest_common_prefix(&["dog","race_car","car"]), "");
        assert_eq!(Solution1::longest_common_prefix(&["same","same","same"]), "same");
        assert_eq!(Solution1::longest_common_prefix(&["test"]), "test");
        assert_eq!(Solution1::longest_common_prefix(&[]), "");
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::longest_common_prefix(&["flower","flow","flight"]));
    }
}