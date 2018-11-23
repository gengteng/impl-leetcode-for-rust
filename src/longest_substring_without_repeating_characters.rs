/// # 3. Longest Substring Without Repeating Characters
///
/// Given a string, find the length of the longest substring without repeating characters.
///
/// # Examples:
///
/// Given "abcabcbb", the answer is "abc", which the length is 3.
///
/// Given "bbbbb", the answer is "b", with the length of 1.
///
/// Given "pwwkew", the answer is "wke", with the length of 3. Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
///
pub trait LongestSubstringWithoutRepeatingCharacters {
    fn length_of_longest_substring(s: &str) -> u32;
}

pub struct Solution1;

impl LongestSubstringWithoutRepeatingCharacters for Solution1 {
    fn length_of_longest_substring(s: &str) -> u32 {
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

pub struct Solution2;

impl LongestSubstringWithoutRepeatingCharacters for Solution2 {
    fn length_of_longest_substring(s: &str) -> u32 {
        use std::collections::HashSet;

        let v = s.as_bytes();
        let len = s.len();
        let mut max = 0;
        let mut set = HashSet::new();
        let mut i = 0;
        let mut j = 0;

        while i < len && j < len {
            let b = v[j];
            if set.contains(&b) {
                set.remove(&v[i]);
                i += 1;
            } else {
                set.insert(b);
                j += 1;

                let n = (j - i) as u32;
                max = if n > max { n  } else { max };
            }
        }

        max
    }
}

pub struct Solution3;

impl LongestSubstringWithoutRepeatingCharacters for Solution3 {
    fn length_of_longest_substring(s: &str) -> u32 {
        use std::collections::HashMap;
        
        let v = s.as_bytes();
        let mut max = 0;
        let mut map = HashMap::new();
        let mut i =0;

        for (j, c) in v.iter().enumerate() {
            match map.get(c) {
                Some(&n) => {
                    i = if n > i { n } else { i };
                },
                None => {

                }
            }

            let len = (j - i + 1) as u32;
            max = if len > max { len } else { max };
            map.insert(c, j + 1);
        }
        
        max
    }
}

#[cfg(test)]
mod test {
    use super::LongestSubstringWithoutRepeatingCharacters;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(Solution1::length_of_longest_substring("bbbbbbb"), 1);
        assert_eq!(Solution1::length_of_longest_substring("pwwkew"), 3);
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::length_of_longest_substring("abcabcbb"));
    }

    use super::Solution2;
    #[test]
    fn test_solution2() {
        assert_eq!(Solution2::length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(Solution2::length_of_longest_substring("bbbbbbb"), 1);
        assert_eq!(Solution2::length_of_longest_substring("pwwkew"), 3);
    }

    #[bench]
    fn bench_solution2(b: &mut Bencher) {
        b.iter(|| Solution2::length_of_longest_substring("abcabcbb"));
    }

    use super::Solution3;
    #[test]
    fn test_solution3() {
        assert_eq!(Solution3::length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(Solution3::length_of_longest_substring("bbbbbbb"), 1);
        assert_eq!(Solution3::length_of_longest_substring("pwwkew"), 3);
    }

    #[bench]
    fn bench_solution3(b: &mut Bencher) {
        b.iter(|| Solution3::length_of_longest_substring("abcabcbb"));
    }

}