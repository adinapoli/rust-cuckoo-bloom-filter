pub mod cuckoo_bloom_filter {

use std::hash::{Hash, hash};

#[deriving(PartialEq, Eq)]
pub struct Fingerprint {
    fingerprint: u64
}

impl Fingerprint {
    // Creates a new Fingerprint from the given item.
    // This is NOT the correct implementation. Fix it.
    pub fn new<'a, K: Hash>(item: &'a K) -> Fingerprint {
        Fingerprint { fingerprint: hash(item) }
    }
}

pub struct Entry {
    value: Option<Fingerprint>
}

impl Entry {
    pub fn new() -> Entry { Entry { value: None } }
    pub fn set_value(&mut self, new_value: Option<Fingerprint>) {
        self.value = new_value
    }
}

pub struct FilterSettings {
    kick_factor: uint
}

impl FilterSettings {
    pub fn def() -> FilterSettings { FilterSettings { kick_factor: 500 } }
}

pub struct CuckooBloomFilter<V> {
    size: u64,
    buckets: Vec<Entry>,
    settings: FilterSettings
}

impl<V: Hash> CuckooBloomFilter<V> {
  pub fn new(size: u64) -> CuckooBloomFilter<V> {
      let s: FilterSettings = FilterSettings::def();
      let f: CuckooBloomFilter<V> = CuckooBloomFilter::<V>::new_with_settings(size, s);
      f
  }

    pub fn new_with_settings(size: u64, settings: FilterSettings) -> CuckooBloomFilter<V> {
        CuckooBloomFilter {
            size: size,
            buckets: Vec::from_fn(size as uint, |_| Entry { value: None }),
            settings: settings
        }
    }

    pub fn insert<V: Hash>(&self, x: &V) -> Result<(), CuckooFailure> {
        let f  = Fingerprint::new(x);
        let i1 = hash(x);
        let _ = i1 ^ f.fingerprint;
        Err(CuckooFailure::Full)
    }

    fn contains_fingerprint<'a>(& self
                                , idx: u64
                                , fin: &'a Fingerprint) -> bool {
        if idx >= self.size { return false }
        let ref e: Entry = self.buckets[idx as uint];
            match (*e).value {
                None => false,
                Some(ref v) => if *v == *fin { true } else { false }
            }
    }

   pub fn lookup<V: Hash>(&self, x: &V) -> bool {
       let f  = Fingerprint::new(x);
       let i1 = hash(x);
       let i2 = i1 ^ f.fingerprint;

       if self.contains_fingerprint(i1, &f) {
           return true
       } else if self.contains_fingerprint(i2, &f) {
           return true
       }

       return false
   }

}

pub enum CuckooFailure {
    Full
}

}

#[cfg(test)]
mod test {
    use cuckoo_bloom_filter::{FilterSettings, CuckooBloomFilter};
    use std::string::{String};

    #[test]
    fn insert_on_singleton() {
        let s = FilterSettings::def();
        let v = "hello".to_string();
        let f = CuckooBloomFilter::<String>::new_with_settings(1, s);
        f.insert(&v);
        assert_eq!(f.lookup(&v), true);
    }
}
