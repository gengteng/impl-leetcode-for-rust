/// # 6. ZigZag Conversion
///
/// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
/// And then read line by line: "PAHNAPLSIIGYIR"
///
/// Write the code that will take a string and make this conversion given a number of rows:
///
/// string convert(string s, int numRows);
/// Example 1:
///
/// Input: s = "PAYPALISHIRING", numRows = 3
/// Output: "PAHNAPLSIIGYIR"
/// Example 2:
///
/// Input: s = "PAYPALISHIRING", numRows = 4
/// Output: "PINALSIGYAHRPI"
/// Explanation:
///
/// P     I    N
/// A   L S  I G
/// Y A   H R
/// P     I
pub trait ZigZagConversion {
    fn convert(s: &str, num_rows: usize) -> String;
}

pub struct Solution1;


impl ZigZagConversion for Solution1 {
    fn convert(s: &str, num_rows: usize) -> String {
        let mut v = Vec::with_capacity(num_rows);
        v.resize_with(num_rows, String::new);

        let mut r = 0;
        let mut c = 0;
        for b in s.chars()  {
            v.get_mut(r).unwrap().push(b);
            if r == num_rows - 1 || c % (num_rows - 1) != 0 {
                r -= 1;
                c += 1;
            } else {
                r += 1;
            }
        }

        let mut result = String::new();
        for line in v.iter() {
            result.push_str(line);
        }

        result
    }
}

#[test]
fn test_solution1() {
    assert_eq!(Solution1::convert("LEETCODEISHIRING", 4), "LDREOEIIECIHNTSG");
    assert_eq!(Solution1::convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    assert_eq!(Solution1::convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
}