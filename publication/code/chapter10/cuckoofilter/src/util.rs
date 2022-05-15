use std::hash::{Hash, Hasher};
use byteorder::{BigEndian, WriteBytesExt};
use crate::bucket::{FigurePrint, FIGERPRINT_SIZE};

struct FaI {
    fp: FigurePrint,
    i1: usize,
    i2: usize,
}

impl FaI {
    fn from_data<T: ?Sized + Hash, H: Hasher + Default>(elem: &T) -> Self {
        let (fp_hash, index_hash) = self.get_hash::<_, H>(elem);
        let mut fp_hash_arr = [0; FIGERPRINT_SIZE];
        let _ = (&mut fp_hash_arr[..]).write_u32::<BigEndian>(fp_hash);

        let fp;
        let mut valid_fp_hash: [u8; FIGERPRINT_SIZE] = [0; FIGERPRINT_SIZE];
        let mut n = 0;
        loop {
            for i in 0..FIGERPRINT_SIZE {
                valid_fp_hash[i] = fp_hash_arr[i] + n;
            }

            if let Some(val) = FigurePrint::from_data(valid_fp_hash) {
                fp = val;
                break;
            }

            n += 1;
        }

        let i1 = index_hash as usize;
        let i2 = get_alt_index::<H>(fp, i1);
        Self { fp, i1, i2 }
    }

    fn get_alt_index<H: Hasher + Default>(fp: FigurePrint, i: usize) -> usize {
        let (_, index_hash) = self.get_hash::<_, H>(&fp.data);
        let alt_i = index_hash as usize;
        (i ^ alt_i) as usize
    }

    fn get_hash<T: ?Sized + Hash, H: Hasher + Default>(elem: &T) -> (u32, u32) {
        let mut hasher = <H as Default>::default();
        elem.hash(&mut hasher);
        let res = hasher.finish();
        ((res >> 32) as u32, res as u32)
    }

    fn random_index<R: rand::Rng>(&self, r: &mut R) -> usize {
        if r.gen() {
            self.i1
        } else {
            self.i2
        }
    }
}
