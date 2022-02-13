// base58 编码字符
const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// 进制映射关系
const BASE58_DIGITS_MAP: &'static [u8] = &[
    255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
    255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
    255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
    255,  0,  1,  2,  3,  4,  5,  6,  7,  8,255,255,255,255,255,255,
    255,  9, 10, 11, 12, 13, 14, 15, 16,255, 17, 18, 19, 20, 21,255,
     22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,255,255,255,255,255,
    255, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,255, 44, 45, 46,
     47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,255,255,255,255,255,
];
// 数前面的 0 用 1 代替
const ALPHABET_INDEX_0: char = '1';

// 解码错误类型
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Invalid,
    InvalidLength,
    InvalidCharacter(char, usize),
}

// 编解码 trait
pub trait Encoder {
    fn encode_to_base58(&self) -> String;
}

pub trait Decoder {
    fn decode_from_base58(&self) -> Result<String, DecodeError>;
}

// 实现 base58 编码
impl Encoder for str {
    fn encode_to_base58(&self) -> String {
        let str_u8 = self.as_bytes();
        let zero_count = str_u8.iter().take_while(|&&x| x == 0).count();
        let size = (str_u8.len() - zero_count) * 138 / 100 + 1;

        // 字符进制转换
        let mut i = zero_count;
        let mut high = size - 1;
        let mut buffer = vec![0u8; size];
        while i < str_u8.len() {
            let mut j = size - 1;
            let mut carry = str_u8[i] as u32;

            while j > high || carry != 0 {
                carry += 256 * buffer[j] as u32;
                buffer[j] = (carry % 58) as u8;
                carry /= 58;

                if j  > 0 {
                    j -= 1;
                }
            }

            i += 1;
            high = j;
        }

        // 处理多个前置 0
        let mut base58_str = String::new();
        for _ in 0..zero_count {
            base58_str.push(ALPHABET_INDEX_0);
        }

        // 获取编码后的字符并拼接成字符串
        let mut j = buffer.iter().take_while(|&&x| x == 0).count();
        while j < size {
            base58_str.push(ALPHABET[buffer[j] as usize] as char);
            j += 1;
        }

        base58_str
    }
}

// 实现 base58 解码
impl Decoder for str {
    fn decode_from_base58(&self) -> Result<String, DecodeError> {
        let mut bin = [0u8; 132];
        let mut out = [0u32; (132 + 3) / 4];
        let bytes_left = (bin.len() % 4) as u8;
        let zero_mask = match bytes_left {
            0 => 0u32,
            _ => 0xffffffff << (bytes_left * 8),
        };

        let zero_count = self.chars().take_while(|&x| x == ALPHABET_INDEX_0).count();
        let mut i = zero_count;
        let b58: Vec<u8> = self.bytes().collect();
        while i < self.len() {
            if (b58[i] & 0x80) != 0 {
                return Err(DecodeError::InvalidCharacter(b58[i] as char, i));
            }

            if BASE58_DIGITS_MAP[b58[i] as usize] == 255 {
                return Err(DecodeError::InvalidCharacter(b58[i] as char, i));
            }

            let mut j = out.len();
            let mut c = BASE58_DIGITS_MAP[b58[i] as usize] as u64;
            while j != 0 {
                j -= 1;
                let t = out[j] as u64 * 58 + c;
                c = (t & 0x3f00000000) >> 32;
                out[j] = (t & 0xffffffff) as u32;
            }

            if c != 0 {
                return Err(DecodeError::InvalidLength);
            }
            if (out[0] & zero_mask) != 0 {
                return Err(DecodeError::InvalidLength);
            }

            i += 1;
        }

        let mut i = 1;
        let mut j = 0;
        bin[0] = match bytes_left {
            3 => ((out[0] & 0xff0000) >> 16) as u8,
            2 => ((out[0] & 0xff00) >> 8) as u8,
            1 => {
                j = 1;
                (out[0] & 0xff) as u8
            },
            _ => {
                i = 0;
                bin[0]
            }
        };

        while j < out.len() {
            bin[i] = ((out[j] >> 0x18) & 0xff) as u8;
            bin[i+1] = ((out[j] >> 0x10) & 0xff) as u8;
            bin[i+2] = ((out[j] >> 8) & 0xff) as u8;
            bin[i+3] = ((out[j] >> 0) & 0xff) as u8;
            i += 4;
            j += 1;
        }

        let leading_zeros = bin.iter().take_while(|&&x| x == 0).count();
        let new_str = String::from_utf8(bin[leading_zeros - zero_count..].to_vec());
        match new_str {
            Ok(res) => Ok(res),
            Err(_) => Err(DecodeError::Invalid),
        }
    }
}

fn main() {
    println!("{:#?}","abc".encode_to_base58());
    println!("{:#?}","ZiCa".decode_from_base58().unwrap());
    println!("{:#?}","我愛你iloveu".encode_to_base58());
    println!("{:#?}","7T5VrPqoBr9DeUXiUr2Fn".decode_from_base58().unwrap());
}
