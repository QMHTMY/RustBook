const FIGERPRINT_SIZE: usize = 1;
const BUCKET_SIZE: usize = 4;

// 指纹定义
#[derive(Debug)]
struct FingerPrint {
    data: [u8; FIGERPRINT_SIZE],
}

// 指纹
impl FingerPrint {
    // 转换函数，从 &[u8; N] -> FingerPrint
    fn from_data(data: &[u8; FIGERPRINT_SIZE]) -> Option<Self> {
        let res = Self { data };
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    fn empty() -> Self {
        Self { data: [100; FIGERPRINT_SIZE] }
    }

    fn is_empty(&self) -> bool {
        [100; FIGERPRINT_SIZE] == self.data
    }
}

// 桶
#[derive(Debug)]
struct Bucket {
    buffer: [FingerPrint; BUCKET_SIZE],
}

impl Bucket {
    fn new() -> Self {
        Self { buffer: [FingerPrint::empty(); BUCKET_SIZE] }
    }

    fn insert(&mut self, fp: FingerPrint) -> bool {
        for bfp in &mut self.buffer {
            if bfp.is_empty() {
                *bfp = fp;
                return true;
            }
        }

        false
    }

    fn delete(&mut self, fp: FingerPrint) -> bool {
        match self.get_fp_index(fp) {
            Some(index) => self.buffer[index] = FingerPrint::empty(),
            None => false,
        }
    }

    // 获取指纹索引下标
    fn get_fp_index(&self, fp: FingerPrint) -> Option<usize> {
        self.buffer.iter().position(|&i| i == fp)
    }
}
