use std::hash::{Hash, hash};

#[deriving(PartialEq, Eq)]
struct Fingerprint {
    getFingerprint: u64
}

impl Fingerprint {
    // Creates a new Fingerprint from the given item.
    // This is NOT the correct implementation. Fix it.
    pub fn new<'a, K: Hash>(item: &'a K) -> Fingerprint {
        Fingerprint { getFingerprint: hash(item) }
    }

    pub fn getFingerprint(&self) -> u64 { self.getFingerprint }
}

struct Entry {
    value: Option<Fingerprint>
}

impl Entry {
    pub fn new() -> Entry { Entry { value: None } }
}

struct FilterSettings {
    kickFactor: uint
}

impl FilterSettings {
    fn def() -> FilterSettings { FilterSettings { kickFactor: 500 } }
}

struct CuckooBloomFilter<V> {
    size: u64,
    buckets: Vec<Entry>,
    settings: FilterSettings
}

impl<V> CuckooBloomFilter<V> {
   //pub fn new<V: Hash>(size: u64) -> CuckooBloomFilter<V> {
   //    let s: FilterSettings = FilterSettings::def();
   //    let f: CuckooBloomFilter<V> = CuckooBloomFilter::newWithSettings(size, s);
   //    f
   //}

    pub fn newWithSettings<V: Hash>(size: u64, settings: FilterSettings) -> CuckooBloomFilter<V> {
        CuckooBloomFilter {
            size: size,
            buckets: range(0u64, size).map(|_| Entry::new()).collect(),
            settings: settings
        }
    }

    pub fn insert<V: Hash>(&self, x: V) -> Result<(), CuckooFailure> {
        let f  = Fingerprint::new(&x);
        let i1 = hash(&x);
        let i2 = i1 ^ f.getFingerprint();
        Err(CuckooFailure::Full)
    }

    fn contains_fingerprint(&self, idx: u64, fin: Fingerprint) -> Option<&Entry> {
        if idx >= self.size { return None }
        let e: &Entry = &self.buckets[idx as uint];
        match (*e).value {
            None => None,
            Some(v) => if v == fin { Some(e) } else { None }
        }
    }

    pub fn lookup<V: Hash>(&self, x: V) -> bool {
        let f  = Fingerprint::new(&x);
        let i1 = hash(&x);
        let i2 = i1 ^ f.getFingerprint();
        false
    }

}

enum CuckooFailure {
    Full
}

#[test]
fn it_works() {
}
