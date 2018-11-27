/// # 12. Integer to Roman
///
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
/// For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
///
/// I can be placed before V (5) and X (10) to make 4 and 9.
/// X can be placed before L (50) and C (100) to make 40 and 90.
/// C can be placed before D (500) and M (1000) to make 400 and 900.
/// Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.
///
/// # Example 1:
///
/// Input: 3
/// Output: "III"
/// # Example 2:
///
/// Input: 4
/// Output: "IV"
/// # Example 3:
///
/// Input: 9
/// Output: "IX"
/// # Example 4:
///
/// Input: 58
/// Output: "LVIII"
/// Explanation: C = 100, L = 50, XXX = 30 and III = 3.
/// # Example 5:
///
/// Input: 1994
/// Output: "MCMXCIV"
/// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
pub trait IntegerToRoman {
    fn int_to_roman(num: u32) -> String;
}

pub struct Solution1;

impl IntegerToRoman for Solution1 {
    fn int_to_roman(num: u32) -> String {
        let mut x = num;
        let mut result = String::with_capacity(12);;

        let table = [("M", 1000u32),
            ("CM", 900), ("D", 500), ("CD", 400), ("C", 100),
            ("XC", 90), ("L", 50), ("XL", 40), ("X", 10),
            ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)
        ];

        for (s, i) in table.iter() {
            while x >= *i {
                x -= i;
                result.push_str(s);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::IntegerToRoman;
    use test::Bencher;

    use super::Solution1;
    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::int_to_roman(3), "III");
        assert_eq!(Solution1::int_to_roman(4), "IV");
        assert_eq!(Solution1::int_to_roman(9), "IX");
        assert_eq!(Solution1::int_to_roman(58), "LVIII");
        assert_eq!(Solution1::int_to_roman(1994), "MCMXCIV");
        assert_eq!(Solution1::int_to_roman(3333), "MMMCCCXXXIII");
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| Solution1::int_to_roman(1994));
    }
}