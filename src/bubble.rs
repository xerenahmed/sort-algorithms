pub mod bubble {
    pub fn bubble_sort<T: Ord + Copy + Clone>(vals: &mut Vec<T>) -> Vec<T> {
        let mut changed = false;

        for i in 1..vals.len() {
            let current = vals[i];
            let before = vals[i - 1];
            if current > before {
                vals[i - 1] = current;
                vals[i] = before;
                changed = true
            }
        }

        if changed {
            bubble_sort(vals)
        } else {
            vals.to_vec()
        }
    }
}
