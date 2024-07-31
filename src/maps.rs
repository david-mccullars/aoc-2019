use hashbrown::hash_map::Iter;
use hashbrown::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::hash::Hash;
use std::ops::Range;

#[derive(Clone, Default)]
pub struct BiMap<K, V> {
    map: HashMap<K, V>,
    inverted: HashMap<V, K>,
}

impl<K: Eq + Clone + Hash, V: Eq + Clone + Hash> BiMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            inverted: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inverted.insert(value.clone(), key.clone());
        self.map.insert(key, value)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn contains_value(&self, value: &V) -> bool {
        self.inverted.contains_key(value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn iget(&self, value: &V) -> Option<&K> {
        self.inverted.get(value)
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.map.iter()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K: Debug, V: Debug> Debug for BiMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.map.fmt(f)
    }
}

#[derive(Default)]
pub struct IdMap<T: Hash + PartialEq>(BiMap<T, usize>);

impl<T: Clone + Eq + Hash + PartialEq> IdMap<T> {
    pub fn new() -> Self {
        Self(BiMap::new())
    }

    pub fn id(&mut self, index: &T) -> usize {
        if let Some(v) = self.0.get(index) {
            *v
        } else {
            let v = self.0.len();
            self.0.insert(index.clone(), v);
            v
        }
    }

    pub fn key(&self, id: &usize) -> Option<&T> {
        self.0.iget(id)
    }

    pub fn ids(&self) -> Range<usize> {
        0..self.0.len()
    }
}
