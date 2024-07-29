pub struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        use std::iter;
        let mut carry = 0;
        let mut cur_sum = 0;
        let mut char_vec = a
            .as_bytes()
            .iter()
            .rev()
            .chain(iter::repeat(&b'0'))
            .zip(b.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
            .take(a.len().max(b.len()))
            .map(|(ac, bc)| {
                cur_sum = (*ac - b'0') + (*bc - b'0') + carry;
                carry = cur_sum / 2;
                match cur_sum % 2 {
                    1 => '1',
                    _ => '0',
                }
            })
            .collect::<Vec<_>>();

        if carry == 1 {
            char_vec.push('1');
        }

        char_vec.iter().rev().collect()
    }
}

fn main() {
    let a = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string();
    let b = "1".to_string();

    let expected = "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
    let result = Solution::add_binary(a, b);

    assert_eq!(expected, result);
}
