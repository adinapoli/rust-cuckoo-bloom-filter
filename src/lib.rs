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
    pub fn setValue(&mut self, newValue: Option<Fingerprint>) {
        self.value = newValue
    }
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
            buckets: Vec::with_capacity(size as uint), //dangerous cast?
            settings: settings
        }
    }

    pub fn insert<V: Hash>(&self, x: V) -> Result<(), CuckooFailure> {
        let f  = Fingerprint::new(&x);
        let i1 = hash(&x);
        let i2 = i1 ^ f.getFingerprint();
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

   pub fn lookup<V: Hash>(&self, x: V) -> bool {
       let f  = Fingerprint::new(&x);
       let i1 = hash(&x);
       let i2 = i1 ^ f.getFingerprint();

       if self.contains_fingerprint(i1, &f) {
           return true
       } else if self.contains_fingerprint(i2, &f) {
           return true
       }

       return false
   }

}

enum CuckooFailure {
    Full
}

#[test]
fn it_works() {
}
