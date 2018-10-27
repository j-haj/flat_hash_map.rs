use std::usize;
use std::hash::Hash;

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for i32 {
    fn hash(&self) -> usize { *self as usize }
}

impl Hashable for i64 {
    fn hash(&self) -> usize { *self as usize }
}

#[derive(Copy,Default)]
struct MapItem<K, V> where K: Clone+Default+Eq+Hash, V: Default+Clone {
    key: K,
    value: V,
}

impl<K, V> Clone for MapItem<K, V> where K: Clone+Default+Eq+Hash, V: Default+Clone {
    fn clone(&self) -> MapItem<K, V> {
        MapItem{key: self.key.clone(), value: self.value.clone()}
    }
}

struct FlatHashMap<K, V> where K: Clone+Default+Eq+Hash, V: Default+Clone {
    key_bits: usize,
    load_factor: f32,
    size: usize,
    data: Vec<Vec<MapItem<K, V>>>,
}

impl<K:, V> FlatHashMap<K, V> where K: Clone+Default+Eq+Hash, V: Default+Clone {
    fn new() -> FlatHashMap<K, V> {
        let mut m: FlatHashMap<K, V> = FlatHashMap{key_bits: 64, load_factor: 0.5, size: 0, data: Vec::with_capacity(1)};
        m.data.resize(1, Default::default());
        m
    }

    fn fhash(&self, x: usize) -> usize {
        x.wrapping_mul(0x9E3779B97F4A7C15usize) >> (64 - self.key_bits)
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.load() > self.load_factor {
            let c = self.capacity();
            self.data.resize(2 * c / self.load_factor as usize, Default::default());
        }
        let k = self.fhash(key.hash()) % self.capacity();
        if k >= self.size {
            let n = self.capacity();
            self.data.resize(n, Default::default());
        }
        self.data[k].append(MapItem{key: key, value: value});
        self.size += 1;
    }

    pub fn get(&self, key: K) -> Option<V> {
        let k = self.fhash(key.hash()) % self.capacity();
        for p in self.data[k].iter() {
            if p.key == key {
                return Some(V);
            }
        }
        return None;
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn load(&self) -> f32 {
        self.size as f32 / self.capacity() as f32
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn insert_empty() {
        let mut m = FlatHashMap<i64, String>::new();
        assert_eq(m.len(), 0);
        m.insert(5, "five");
        assert_eq(m.len(), 1);
    }
}
