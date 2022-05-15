mod bucket;
mod util;

use std::fmt;
use std::cmp::max;
use std::iter::repeat;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::collections::hash_map::DefaultHasher;

// 序列化
use rand::Rng;
#[cfg(feature = "serde_support")]
use serde_derive::{Serialize, Deserialize};

use crate::util::FaI;
use crate::bucket::{Bucket, FingerPrint, BUCKET_SIZE, FIGERPRINT_SIZE};

const MAX_RELOCATION: u32 = 100;
const DEFAULT_CAPACITY: usize = (1 << 20) - 1;

// 错误处理
#[derive(Debug)]
enum CuckooError {
    NotEnoughSpace,
}

// 添加打印输出功能
impl fmt::Display for CuckooError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NotEnoughSpace")
    }
}

impl Error for CuckooError {
    fn description(&self) -> &str {
        "Not enough space to save new element, operation failed!"
    }
}

// 布谷鸟过滤器
struct CuckooFilter<H> {
    buckets: Box<[Bucket]>,   // 桶
    len: usize,               // 长度
    _phantom: PhantomData<H>,
}

// 添加默认值功能
impl Default for CuckooFilter<DefaultHasher> {
    fn default() -> Self {
        Self::new()
    }
}

impl CuckooFilter<DefaultHasher> {
    fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl<H: Hasher + Default> CuckooFilter<H> {
    fn with_capacity(cap: usize) -> Self {
        let capacity = max(1, cap.next_power_of_two() / BUCKET_SIZE);
        Self {
            buckets: repeat(Bucket::new()) // 构建 capacity 个 Bucket
                    .take(capacity)
                    .collect::<Vec<_>>(),
                    .into_boxed_slice(),
            len: 0,
            _phantom: PhantomData,
        }
    }

    // 尝试插入
    fn try_insert<T: ?Sized + Hash>(&mut self, elem: &T) -> Result<bool, CuckooError> {
        if self.contains(elem) {
            Ok(false)
        } else {
            self.insert(elem).map(|_| true)
        }
    }

    fn insert<T: ?Sized + Hash>(&mut self, elem: &T) -> Result<(), CuckooError> {
        let fai = FaI::from_data::<_, H>(elem)
        if self.put(fai.fp, fai.i1) || self.put(fai.fp, fai.i2) {
            return Ok(());
        }

        // 插入数据冲突，重定位
        let mut rng = rand::thread_rng();
        let mut i = fai.random_index(&mut rng);
        let mut fp = fai.fp;
        for _ in 0..MAX_RELOCATION {
            let other_fp;
            {
                let loc = &mut self.buckets[i % self.len]
                                   .buffer[rng.gen_range(0, BUCKET_SIZE)];
                other_fp = *loc;
                *loc = fp;
                i = FaI::get_alt_index::<H>(other_fp, i);
            }
            if self.put(other_fp, i) {
                return Ok(());
            }
            fp = other_fp;
        }

        Err(CuckooError::NotEnoughSpace)
    }

    fn put(&mut self, fp: FingerPrint, i: usize) -> bool {
        if self.buckets[i % self.len].insert(fp) {
            self.len += 1;
            true
        } else {
            false
        }
    }

    fn delete<T: ?Sized + Hash>(&mut self, elem: &T) -> bool {
        let FaI { fp, i1, i2 } = FaI::from_data::<_, H>(elem);
        self.remove(fp, i1) || self.remove(fp, i2)
    }

    fn remove(&mut self, fp: FingerPrint, i: usize) -> bool {
        if self.buckets[i % self.len].delete(fp) {
            self.len -= 1;
            true
        } else {
            false
        }
    }

    fn contains<T: ?Sized + Hash>(&self, elem: &T) -> bool {
        let FaI { fp, i1, i2 } = FaI::from_data::<_, H>(elem);
        self.buckets[i1 % self.len]
            .get_fp_index(fp)
            .or_else(|| { self.buckets[i2 % self.len].get_fp_index(fp) })
            .is_some()
    }
}
