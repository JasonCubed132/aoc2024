use std::collections::HashMap;

pub fn count_items_in_list<T: Eq + std::hash::Hash>(input: &Vec<T>) -> HashMap<&T, i32> {
    input.into_iter().fold(HashMap::new(), |mut acc, item| {
        acc.entry(&item).and_modify(|val| *val += 1).or_insert(1);
        acc
    })
}
