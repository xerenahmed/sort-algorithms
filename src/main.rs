mod bubble;
mod radix;

use bubble::bubble::bubble_sort;
use radix::radix::radix_sort;

fn main() {
    let vals: Vec<i64> = vec![1, 5, 43, 167, 86, 43, 14, 5, 6, 7, 89, 9, 324];

    println!("Default: {:?}", vals);
    println!("Bubble: {:?}", bubble_sort(&mut vals.clone()));
    println!("Radix: {:?}", radix_sort(&mut vals.clone(), 1));
}
