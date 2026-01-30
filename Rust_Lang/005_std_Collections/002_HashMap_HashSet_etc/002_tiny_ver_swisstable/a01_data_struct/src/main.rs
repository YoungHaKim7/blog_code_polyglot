use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const EMPTY: u8 = 0x80; // same idea as SwissTable
const GROUP_SIZE: usize = 8; // small for learning

pub struct TinySwissMap<K, V> {
    control: Vec<u8>,
    entries: Vec<Option<(K, V)>>,
    mask: usize,
}

impl<K, V> TinySwissMap<K, V>
where
    K: std::clone::Clone,
    V: std::clone::Clone,
{
    pub fn new(capacity: usize) -> Self {
        assert!(capacity.is_power_of_two());

        Self {
            control: vec![EMPTY; capacity],
            entries: vec![None; capacity],
            mask: capacity - 1,
        }
    }
}

impl<K: Eq + Hash, V> TinySwissMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        let hash = hash(&key);
        let mut index = h1(hash) & self.mask;
        let tag = h2(hash);

        loop {
            let group_start = index & !(GROUP_SIZE - 1);

            for i in 0..GROUP_SIZE {
                let idx = (group_start + i) & self.mask;

                if self.control[idx] == EMPTY {
                    self.control[idx] = tag;
                    self.entries[idx] = Some((key, value));
                    return;
                }
            }

            index = (group_start + GROUP_SIZE) & self.mask;
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = hash(key);
        let mut index = h1(hash) & self.mask;
        let tag = h2(hash);

        loop {
            let group_start = index & !(GROUP_SIZE - 1);

            if let Some(pos) = find_in_group(&self.control, group_start, tag) {
                if let Some((k, v)) = &self.entries[pos] {
                    if k == key {
                        return Some(v);
                    }
                }
            } else {
                return None;
            }

            index = (group_start + GROUP_SIZE) & self.mask;
        }
    }
}

fn hash<Q: Hash>(key: &Q) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn h1(hash: u64) -> usize {
    (hash >> 7) as usize
}

fn h2(hash: u64) -> u8 {
    (hash & 0x7F) as u8
}

fn find_in_group(control: &[u8], start: usize, h2: u8) -> Option<usize> {
    for i in 0..GROUP_SIZE {
        let idx = start + i;
        let c = control[idx];
        if c == h2 {
            return Some(idx);
        }
        if c == EMPTY {
            return None; // early exit!
        }
    }
    None
}

fn main() {
    let mut map = TinySwissMap::new(16);

    map.insert("apple", 3);
    map.insert("banana", 5);
    map.insert("orange", 7);

    println!("{:?}", map.get(&"apple")); // Some(3)
    println!("{:?}", map.get(&"banana")); // Some(5)
    println!("{:?}", map.get(&"grape")); // None
}
