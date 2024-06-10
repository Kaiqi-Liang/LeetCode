//! <https://leetcode.com/problems/product-of-array-except-self/>
mod precompute {
    macro_rules! fold_product {
        ($iter:expr) => {
            $iter.fold(Vec::new(), |mut a, c| {
                a.push(if let Some(&prev) = a.last() { prev } else { 1 } * c);
                a
            })
        };
    }
    pub fn product_except_self(integers: Vec<i32>) -> Vec<i32> {
        let prefix = fold_product!(integers.iter());
        let mut postfix = fold_product!(integers.iter().rev());
        postfix.reverse();
        (0..integers.len()).fold(Vec::new(), |mut a, c| {
            a.push(
                if c > 0 { prefix[c - 1] } else { 1 }
                    * if c < integers.len() - 1 {
                        postfix[c + 1]
                    } else {
                        1
                    },
            );
            a
        })
    }
}

mod division {
    pub fn product_except_self(integers: Vec<i32>) -> Vec<i32> {
        let num_of_zeros = integers.iter().filter(|&&integer| integer == 0).count();
        match num_of_zeros.cmp(&1) {
            std::cmp::Ordering::Less => {
                let product: i32 = integers.iter().product();
                integers
                    .into_iter()
                    .map(|integer| product / integer)
                    .collect()
            }
            std::cmp::Ordering::Equal => {
                let mut res = vec![0; integers.len()];
                res[integers
                    .iter()
                    .position(|&integer| integer == 0)
                    .expect("num_of_zeros == 1")] =
                    integers.iter().filter(|&&integer| integer != 0).product();
                res
            }
            std::cmp::Ordering::Greater => vec![0; integers.len()],
        }
    }
}

fn main() {
    assert_eq!(
        precompute::product_except_self(vec![1, 2, 3, 4]),
        division::product_except_self(vec![1, 2, 3, 4])
    );
    assert_eq!(
        precompute::product_except_self(vec![-1, 1, 0, -3, 3]),
        division::product_except_self(vec![-1, 1, 0, -3, 3]),
    );
}
