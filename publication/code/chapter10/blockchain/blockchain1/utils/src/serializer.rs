use bincode;
use serde::Serialize;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

// 序列化数据
pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    bincode::serialize(value).unwrap()
}

// 计算 value 哈希值并以 String 形式返回
pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}
