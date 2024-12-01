use std::collections::HashMap;

pub fn count_items_in_list<T: Eq + std::hash::Hash>(input: &Vec<T>) -> HashMap<&T, i32> {
    let mut counts = HashMap::new();

    for item in input {
        let existing_value = counts.get(&item);
        let new_value;
        match existing_value {
            Some(x) => {
                new_value = x + 1;
            }
            None => {
                new_value = 1;
            }
        }
        counts.insert(item, new_value);
    }

    counts
}
