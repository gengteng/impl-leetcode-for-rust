/// # 8. String to Integer (atoi)
///
/// Implement atoi which converts a string to an integer.
///
/// The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
///
/// The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
///
/// If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
///
/// If no valid conversion could be performed, a zero value is returned.
///
/// # Note:
///
/// Only the space character ' ' is considered as whitespace character.
/// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. If the numerical value is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231) is returned.
/// # Example 1:
///
/// Input: "42"
/// Output: 42
/// # Example 2:
///
/// Input: "   -42"
/// Output: -42
/// Explanation: The first non-whitespace character is '-', which is the minus sign.
///              Then take as many numerical digits as possible, which gets 42.
/// # Example 3:
///
/// Input: "4193 with words"
/// Output: 4193
/// Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
/// # Example 4:
///
/// Input: "words and 987"
/// Output: 0
/// Explanation: The first non-whitespace character is 'w', which is not a numerical
///              digit or a +/- sign. Therefore no valid conversion could be performed.
/// # Example 5:
///
/// Input: "-91283472332"
/// Output: -2147483648
/// Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
///              Thefore INT_MIN (−231) is returned.
///
pub trait StringToInteger {
    fn atoi(s: &str) -> i32;
}

pub struct Solution1;

impl StringToInteger for Solution1 {
    fn atoi(s: &str) -> i32 {
        use std::i32::MAX;
        use std::i32::MIN;

        let chars = s.chars();
        let mut first = false;
        let mut negative = false;
        let mut result = 0;

        for c in chars {
            if first {
                let v = c as i32 - '0' as i32;

                if v < 0 || v > 9 {
                    return result;
                }

                if negative {
                    if result > 0 {
                        result = -result;
                    }

                    if (MIN + v) / 10 <= result {
                        result = result * 10 - v;
                    } else {
                        return MIN;
                    }

                } else { // positive
                    if (MAX - v) / 10 >= result {
                        result = result * 10 + v;
                    } else {
                        return MAX;
                    }
                }
            } else {
                match c {
                    ' ' => continue,
                    '+' => first = true,
                    '-' => {
                        first = true;
                        negative = true;
                    },
                    '0'..='9' => {
                        first = true;
                        result = c as i32 - '0' as i32;
                    },
                    _ => return 0
                }
            }
        }

        result
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::atoi("+1234"), 1234);
    assert_eq!(Solution1::atoi("-1234"), -1234);
    assert_eq!(Solution1::atoi("1234"), 1234);
    assert_eq!(Solution1::atoi("1234 end"), 1234);
    assert_eq!(Solution1::atoi("wtf 1234"), 0);
    assert_eq!(Solution1::atoi("   1234 aha"), 1234);
    assert_eq!(Solution1::atoi("   +1234 123"), 1234);
    assert_eq!(Solution1::atoi("   -1234 dsa"), -1234);
    assert_eq!(Solution1::atoi("   -1234 dsa"), -1234);
    assert_eq!(Solution1::atoi("  +99999999999 max"), std::i32::MAX);
    assert_eq!(Solution1::atoi("   -91283472332 min"), std::i32::MIN);
}