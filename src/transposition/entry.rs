use crate::transposition::bucket::Bucket;
use crate::transposition::defs::HashData;

pub const NR_OF_BUCKETS: usize = 4;

#[derive(Clone)]
pub struct Entry<T> {
    entry: [Bucket<T>; NR_OF_BUCKETS],
}

impl<T> Entry<T>
where
    T: HashData + Copy,
{
    pub fn new() -> Self {
        Self {
            entry: [Bucket::new(); NR_OF_BUCKETS],
        }
    }

    pub fn store(&mut self, verification: u32, data: T, used_buckets: &mut usize) {
        let mut idx_with_lowest_depth = 0;
        for i in 0..NR_OF_BUCKETS {
            if self.entry[i].data.depth() < self.entry[idx_with_lowest_depth].data.depth() {
                idx_with_lowest_depth = i;
            }
        }

        if self.entry[idx_with_lowest_depth].verification == 0 {
            *used_buckets += 1;
        }
        self.entry[idx_with_lowest_depth] = Bucket { verification, data };
    }

    pub fn find_data(&self, verification: u32) -> Option<&T> {
        for bucket in self.entry.iter() {
            if bucket.verification == verification {
                return Some(&bucket.data);
            }
        }
        None
    }
}
