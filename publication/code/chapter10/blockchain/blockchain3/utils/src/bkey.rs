use std::mem::transmute;
use bigint::U256;
use db_key::Key;

// 定义大 Key
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BKey {
    pub val: U256,
}

impl Key for BKey {
    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, func: F) -> T {
        let val = unsafe {
            transmute::<_, &[u8; 32]>(self)
        };
        func(val)
    }

    fn from_u8(key: &[u8]) -> Self {
        assert!(key.len() == 32);
        let mut res: [u8; 32] = [0; 32];

        for (i, val) in key.iter().enumerate() {
            res[i] = *val;
        }

        unsafe {
            transmute::<[u8; 32], Self>(res)
        }
    }
}
