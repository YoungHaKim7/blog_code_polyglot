use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

type Freq = u64;

/// Using VecDeque instead of LinkedList for better stable API support
type ElementList<K, V> = VecDeque<Entry<K, V>>;
type FrequencyList<K, V> = VecDeque<(Freq, ElementList<K, V>)>;

/// Entry<K, V> == C++ struct Entry<K, V>
#[derive(Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct LFUCache<K, V> {
    /// key -> frequency
    cache_map: HashMap<K, Freq>,

    /// Ordered by increasing frequency (front = least freq)
    elements: FrequencyList<K, V>,

    max_size: usize,
    cur_size: usize,
}

impl<K, V> LFUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(size: usize) -> Self {
        Self {
            cache_map: HashMap::new(),
            elements: VecDeque::new(),
            max_size: if size > 0 { size } else { 10 },
            cur_size: 0,
        }
    }

    /* ---------------- internal helpers ---------------- */

    fn increment_frequency(&mut self, key: &K) {
        let freq = *self.cache_map.get(key).unwrap();
        let new_freq = freq + 1;

        // Find and remove the entry from the old frequency list
        let mut entry: Option<Entry<K, V>> = None;
        let mut freq_idx: Option<usize> = None;

        for (i, (f, list)) in self.elements.iter_mut().enumerate() {
            if *f == freq {
                freq_idx = Some(i);
                // Find and remove the entry with matching key
                for j in 0..list.len() {
                    if &list[j].key == key {
                        entry = Some(list.remove(j).unwrap());
                        break;
                    }
                }
                break;
            }
        }

        let entry = entry.unwrap();

        // Remove empty frequency list
        if let Some(idx) = freq_idx {
            if self.elements[idx].1.is_empty() {
                self.elements.remove(idx);
            }
        }

        // Insert into new frequency list
        let mut inserted = false;
        for (f, list) in self.elements.iter_mut() {
            if *f == new_freq {
                list.push_front(entry.clone());
                self.cache_map.insert(key.clone(), new_freq);
                inserted = true;
                break;
            }
        }

        if !inserted {
            // Find the correct position to insert (sorted by frequency)
            let mut insert_idx = 0;
            for (i, (f, _)) in self.elements.iter().enumerate() {
                if *f > new_freq {
                    insert_idx = i;
                    break;
                }
                insert_idx = i + 1;
            }

            let mut new_list = VecDeque::new();
            new_list.push_front(entry);
            self.elements.insert(insert_idx, (new_freq, new_list));
            self.cache_map.insert(key.clone(), new_freq);
        }
    }

    fn erase_old_element(&mut self) {
        if let Some((_, list)) = self.elements.front_mut() {
            if let Some(entry) = list.pop_back() {
                self.cache_map.remove(&entry.key);
                self.cur_size -= 1;
            }
            if list.is_empty() {
                self.elements.pop_front();
            }
        }
    }

    /* ---------------- public API ---------------- */

    pub fn set(&mut self, key: K, value: V) {
        if self.cache_map.contains_key(&key) {
            // update value
            let freq = self.cache_map[&key];
            for (f, list) in self.elements.iter_mut() {
                if *f == freq {
                    for e in list.iter_mut() {
                        if e.key == key {
                            e.value = value.clone();
                            break;
                        }
                    }
                    break;
                }
            }
            self.increment_frequency(&key);
            return;
        }

        if self.cur_size == self.max_size {
            self.erase_old_element();
        }

        // insert with frequency 1
        if let Some((f, list)) = self.elements.front_mut() {
            if *f == 1 {
                list.push_front(Entry {
                    key: key.clone(),
                    value,
                });
            } else {
                let mut list = VecDeque::new();
                list.push_front(Entry {
                    key: key.clone(),
                    value,
                });
                self.elements.push_front((1, list));
            }
        } else {
            let mut list = VecDeque::new();
            list.push_front(Entry {
                key: key.clone(),
                value,
            });
            self.elements.push_front((1, list));
        }

        self.cache_map.insert(key, 1);
        self.cur_size += 1;
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        if !self.cache_map.contains_key(&key) {
            return None;
        }

        let freq = self.cache_map[&key];
        let mut value = None;

        for (f, list) in self.elements.iter() {
            if *f == freq {
                for e in list.iter() {
                    if e.key == key {
                        value = Some(e.value.clone());
                        break;
                    }
                }
                break;
            }
        }

        self.increment_frequency(&key);
        value
    }
}

fn main() {
    let mut cache = LFUCache::<i64, i64>::new(3);

    cache.set(1, 1);
    cache.set(2, 2);
    cache.set(3, 3);
    cache.set(2, 4);

    let r = cache.get(1);
    assert_eq!(r, Some(1));

    cache.set(4, 5); // evict key 3

    assert_eq!(cache.get(3), None);
    assert_eq!(cache.get(4), Some(5));

    let mut cache2 = LFUCache::<i64, String>::new(2);
    cache2.set(1, "one".to_string());
    cache2.set(2, "two".to_string());

    let r = cache2.get(1);
    assert_eq!(r, Some("one".to_string()));

    cache2.set(3, "three".to_string()); // evict 2

    assert_eq!(cache2.get(2), None);
    assert_eq!(cache2.get(3), Some("three".to_string()));
}
