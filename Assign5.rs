use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K: Ord, V> {
    fn sort_by_key(&mut self);
}

// Implement SortByKey for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        let mut sorted_entries: Vec<_> = self.drain().collect();
        sorted_entries.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));
        self.extend(sorted_entries);
    }
}

fn main() {
    // Create a new instance of HashMap
    let mut my_map: HashMap<i32, String> = HashMap::new();

    // Add key-value pairs to the map
    my_map.insert(3, "Three".to_string());
    my_map.insert(1, "One".to_string());
    my_map.insert(2, "Two".to_string());

    println!("Before sorting: {:?}", my_map);

    // Sort the map by keys
    my_map.sort_by_key();

    println!("After sorting: {:?}", my_map);
}
