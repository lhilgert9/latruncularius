use crate::board::defs::ZobristKey;
use crate::transposition::bucket::Bucket;
use crate::transposition::defs::HashData;
use crate::transposition::defs::Transposition;
use crate::transposition::entry::Entry;
use crate::transposition::entry::NR_OF_BUCKETS;

const MEGABYTE: usize = 1024 ^ 2;
const HASH_MASK: u64 = 0xFF_FF_FF_FF_00_00_00_00;
const HASH_MASK_SHIFT: u64 = 32;
const VERIFICATION_MASK: u64 = 0x00_00_00_00_FF_FF_FF_FF;

impl<T> Transposition<T>
where
    T: HashData + Copy + Clone,
{
    pub fn new(megabytes: usize) -> Self {
        let (total_entries, total_buckets) = Self::init_values(megabytes);
        Self {
            table: vec![Entry::<T>::new(); total_entries],
            megabytes,
            used_buckets: 0,
            total_entries,
            total_buckets,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.megabytes > 0
    }

    pub fn resize(&mut self, megabytes: usize) {
        if megabytes != self.megabytes {
            let (total_entries, total_buckets) = Self::init_values(megabytes);

            self.table = vec![Entry::<T>::new(); total_entries];
            self.megabytes = megabytes;
            self.used_buckets = 0;
            self.total_entries = total_entries;
            self.total_buckets = total_buckets;
        } else {
            self.clear();
        }
    }

    pub fn insert(&mut self, zobrist_key: ZobristKey, data: T) {
        if self.is_enabled() {
            let index = self.index_from(zobrist_key);
            let verification = self.verification_from(zobrist_key);
            self.table[index].store(verification, data, &mut self.used_buckets);
        }
    }

    pub fn get(&self, zobrist_key: ZobristKey) -> Option<&T> {
        if self.is_enabled() {
            let index = self.index_from(zobrist_key);
            let verification = self.verification_from(zobrist_key);
            self.find_entry(index).find_data(verification)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = Entry::new();
        }
        self.used_buckets = 0;
    }

    pub fn _hash_full(&self) -> u16 {
        if !self.is_enabled() {
            return 0;
        }

        let fraction = self.used_buckets as f64 / self.total_buckets as f64;
        let promille = (fraction * 1000f64).floor();

        promille as u16
    }

    pub fn hash_full_percent(&self) -> u16 {
        if !self.is_enabled() {
            return 0;
        }

        let fraction = self.used_buckets as f64 / self.total_buckets as f64;
        let percent = (fraction * 100f64).floor();

        percent as u16
    }
}

impl<T> Transposition<T>
where
    T: HashData + Clone,
{
    fn find_entry(&self, index: usize) -> &Entry<T> {
        &self.table[index]
    }

    fn index_from(&self, zobrist_key: ZobristKey) -> usize {
        let key = (zobrist_key & HASH_MASK) >> HASH_MASK_SHIFT;
        (key % (self.total_entries as ZobristKey)) as usize
    }

    fn verification_from(&self, zobrist_key: ZobristKey) -> u32 {
        (zobrist_key & VERIFICATION_MASK) as u32
    }

    fn init_values(megabytes: usize) -> (usize, usize) {
        let bucket_size = std::mem::size_of::<Bucket<T>>();
        let entry_size = bucket_size * NR_OF_BUCKETS;
        let total_entries = MEGABYTE / entry_size * megabytes;
        let total_buckets = total_entries * NR_OF_BUCKETS;

        (total_entries, total_buckets)
    }
}
