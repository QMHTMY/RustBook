use bincode;
use serde::Serialize;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

pub fn hash_u8(value: &[u8], mut out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(&mut out);
}
