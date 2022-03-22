// 21 Mart 2022
// Eren Ahmed Akyol

pub mod radix {
    use digits::{BaseCustom, Digits};

    pub fn radix_sort<T: Ord + Copy + Clone + std::fmt::Display + std::fmt::Debug>(
        vals: Vec<T>,
        current_digit: usize,
    ) -> Vec<T> {
        let mut new_vec: Vec<T> = vec![];
        let mut max_digits = 0;

        for i in (0..=9).rev() {
            for j in vals.clone().into_iter() {
                let (d, digits_len) = digit(j, current_digit);
                if max_digits < digits_len {
                    max_digits = digits_len;
                }

                if d == i as u64 {
                    new_vec.push(j)
                }
            }
        }

        if max_digits > current_digit {
            radix_sort(new_vec, current_digit + 1)
        } else {
            new_vec
        }
    }

    fn digit<T: std::fmt::Display>(number: T, digit: usize) -> (u64, usize) {
        let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
        let digits = Digits::new(base10.clone(), number.to_string()).as_mapping_vec();
        let digits_len = digits.len();
        if digit > digits_len {
            return (0, digits_len);
        }

        let idx = digits_len - digit;

        if idx >= digits_len {
            (0, digits_len)
        } else {
            (digits[digits_len - digit], digits_len)
        }
    }
}
