// from https://docs.rs/im/7.1.0/src/im/hashmap/mod.rs.html#584-590
use std::hash::Hash;
use std::collections::HashMap;

/// Merges two hashmaps, using f where the keys exist in both hashmaps
/// * one: The first hashmap
/// * two: The second hashmap
/// * f: What to do when the key exists in both hashmaps
/// * Returns: The union of one and two
pub fn union_of<K, V, F>(one: &HashMap<K, V>, two: &HashMap<K, V>, f: F) -> HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
    F: Fn(V, V) -> V
{
    two
        .iter()
        .fold(one.clone(), |mut m: HashMap<K, V>, (k, v)| {
                m.insert(
                    k.clone(),
                    one.get(k).map(|v1: &V| f(v1.clone(), v.clone())).unwrap_or(v.clone())
                );
                m
            }
        )
}

/// Gets the intersection of two HashMaps, using f to join them
/// * one: The first hashmap
/// * two: The second hashmap
/// * f: What to do when the key exists in both hashmaps
/// * Returns: The intersection of one and two
pub fn intersection_of<K, A, B, C, F>(one: &HashMap<K, A>, two: &HashMap<K, B>, f: F) -> HashMap<K, C>
where
    K: Hash + Eq + Clone,
    A: Clone,
    B: Clone,
    F: Fn(A, B) -> C
{
    let mut out = HashMap::new();
    for (k, v2) in two {
        match one.get(k) {
            None => (),
            Some(v1) => {
                let result = f(v1.clone(), v2.clone());
                out.insert(k.clone(), result);
            }
        }
    }
    out
}