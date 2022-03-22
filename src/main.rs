mod bubble;
mod radix;

use bubble::bubble::bubble_sort;
use radix::radix::radix_sort;

fn main() {
    let vals: Vec<i64> = vec![
        1, 5, 43, 167, 86, 43, 14, 5, 6, 7, 89, 9, 324, 23, 3, 122, 632, 2415, 1553, 2351, 1251,
        56856,
    ];
    let bubble = bubble_sort(vals.clone());
    let radix = radix_sort(vals.clone(), 1);

    println!("Default: {:?}", vals);
    println!("Bubble: {:?}", bubble);
    println!("Radix:  {:?}", radix);

    assert_eq!(bubble, radix);
}
