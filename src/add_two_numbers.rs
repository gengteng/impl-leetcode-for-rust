use std::collections::LinkedList;

/// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// Example:
/// ```
/// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
/// Output: 7 -> 0 -> 8
/// Explanation: 342 + 465 = 807.
/// ```
pub trait AddTwoNumbers {
    fn add_two_numbers(l1: &LinkedList<u32>, l2: &LinkedList<u32>) -> LinkedList<u32>;
}

pub struct Solution1;

impl AddTwoNumbers for Solution1 {
    fn add_two_numbers(l1: &LinkedList<u32>, l2: &LinkedList<u32>) -> LinkedList<u32> {
        let mut result = LinkedList::new();

        let mut carry = false;
        let mut i1 = l1.iter();
        let mut i2 = l2.iter();
        loop {
            let p1 = i1.next();
            let v1 = match p1 {
                Some(&v) => v,
                None => 0
            };

            let p2 = i2.next();
            let v2 = match p2 {
                Some(&v) => v,
                None => 0
            };

            if p1 == None && p2 == None && !carry {
                break;
            }

            let sum = v1 + v2 + if carry { 1 } else { 0 };
            result.push_back(sum % 10);
            carry = sum > 9;
        }

        result
    }
}

#[test]
fn test_solution1() {
    let mut l1 = LinkedList::<u32>::new();
    l1.push_back(2);
    l1.push_back(4);
    l1.push_back(3);

    let mut l2 = LinkedList::<u32>::new();
    l2.push_back(5);
    l2.push_back(6);
    l2.push_back(4);

    let mut result = LinkedList::<u32>::new();
    result.push_back(7);
    result.push_back(0);
    result.push_back(8);

    assert_eq!(Solution1::add_two_numbers(&l1, &l2), result);

    let mut l1 = LinkedList::<u32>::new();
    l1.push_back(9);
    l1.push_back(9);

    let mut l2 = LinkedList::<u32>::new();
    l2.push_back(9);
    l2.push_back(9);
    l2.push_back(9);

    let mut result = LinkedList::<u32>::new();
    result.push_back(8);
    result.push_back(9);
    result.push_back(0);
    result.push_back(1);

    // 99 + 999 = 1098.
    assert_eq!(Solution1::add_two_numbers(&l1, &l2), result);
}