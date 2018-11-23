/// # 7. Reverse Integer
///
/// Given a 32-bit signed integer, reverse digits of an integer.
///
/// # Example 1:
///
/// Input: 123
/// Output: 321
/// # Example 2:
///
/// Input: -123
/// Output: -321
/// # Example 3:
///
/// Input: 120
/// Output: 21
/// # Note:
/// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2^31,  2^31 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
pub trait ReverseInteger {
    fn reverse(x: i32) -> i32;
}

pub struct Solution1;

impl ReverseInteger for Solution1 {
    fn reverse(x: i32) -> i32 {
        let mut result = 0;
        let mut from = x;

        while from != 0 {
            result = result * 10 + from % 10;
            from /= 10;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::ReverseInteger;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::reverse(123), 321);
        assert_eq!(Solution1::reverse(-123), -321);
        assert_eq!(Solution1::reverse(34343), 34343);
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::reverse(34343));
        b.iter(|| Solution1::reverse(-123));
        b.iter(|| Solution1::reverse(123));
    }
}