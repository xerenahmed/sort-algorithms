pub mod bubble {
    pub fn bubble_sort<T: Ord + Copy + Clone + std::fmt::Debug + std::fmt::Display>(
        vals: Vec<T>,
    ) -> Vec<T> {
        let mut changed = false;
        let mut new_vals: Vec<T> = vals.clone();

        for i in 1..new_vals.len() {
            let current = new_vals[i];
            let before = new_vals[i - 1];
            if current > before {
                new_vals[i - 1] = current;
                new_vals[i] = before;
                changed = true;
            }
        }

        if changed {
            bubble_sort(new_vals.clone())
        } else {
            new_vals
        }
    }
}
