//! base58 编码
//base58 是 base64 的改进版，是比特币中使用的一种编码，用于生成比特币钱包地址，由中本聪引入。

///将 base64 中易错字符[0 O I l + /]剔除，剩下编码字符 58 个
///设计Base58主要的目的是：
///1. 避免混淆。在某些字体下，数字0和字母大写O，以及字母大写I和字母小写l会非常相似。
///2. 不使用"+"和"/"的原因是非字母或数字的字符串作为帐号较难被接受。
///3. 编码结果没有标点符号，通常不会被从中间分行。
///4. 大部分软件支持双击选择整个字符串，因为不分行，所以双击字符串就能复制下来。
const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// 编码转换表
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

/// 数前面的 0 用 1 来代替
const ALPHABET_INDEX_0: char = '1';

/// 用于处理解码失败的错误类型
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Invalid,
    InvalidLength,
    InvalidCharacter(char, usize),
}

// 编码合解码 trait
pub trait Encoder {
    fn encode_to_base58(&self) -> String;
}

pub trait Decoder {
    fn decode_from_base58(&self) -> Result<String, DecodeError>;
}

impl Encoder for str {
    /// 将 str/String 编码为 base58
    ///
    /// # Example
    /// ```
    /// use encoder::base58::Encoder;
    ///
    /// let src = "abc";
    /// let des = "ZiCa";
    /// assert_eq!(des, src.encode_to_base58());
    /// ```
    fn encode_to_base58(&self) -> String {
        let str_u8 = self.as_bytes();
        let zcount = str_u8.iter().take_while(|&&x| x == 0).count();
        let size = (str_u8.len() - zcount) * 138 / 100 + 1;

        let mut i = zcount;
        let mut high = size - 1;
        let mut buffer = vec![0u8; size];
        while i < str_u8.len() {
            let mut j = size - 1;
            let mut carry = str_u8[i] as u32;

            while j > high || carry != 0 {
                carry += 256 * buffer[j] as u32;
                buffer[j] = (carry % 58) as u8;
                carry /= 58;

                if j > 0 {
                    j -= 1;
                }
            }

            i += 1;
            high = j;
        }

        let mut base58_str = String::new();
        for _ in 0..zcount {
            base58_str.push(ALPHABET_INDEX_0);
        }

        let mut j = buffer.iter().take_while(|&&x| x == 0).count();
        while j < size {
            base58_str.push(ALPHABET[buffer[j] as usize] as char);
            j += 1;
        }

        base58_str
    }
}

impl Decoder for str {
    /// 将 base58 编码解码为 String
    ///
    /// # Example
    /// ```
    /// use encoder::base58::Decoder;
    ///
    /// let src = "7T5VrPqoBr9DeUXiUr2Fn";
    /// let des = "我爱你iloveu";
    /// assert_eq!(des, src.decode_from_base58().unwrap());
    /// ```
    fn decode_from_base58(&self) -> Result<String, DecodeError> {
        let mut bin = [0u8; 132];
        let mut out = [0u32; (132 + 3) / 4];
        let bytesleft = (bin.len() % 4) as u8;
        let zeromask = match bytesleft {
            0 => 0u32,
            _ => 0xffffffff << (bytesleft * 8),
        };

        let zcount = self.chars().take_while(|&x| x == ALPHABET_INDEX_0).count();
        let mut i = zcount;
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

            if (out[0] & zeromask) != 0 {
                return Err(DecodeError::InvalidLength);
            }

            i += 1;
        }

        let mut i = 1;
        let mut j = 0;
        bin[0] = match bytesleft {
            3 => ((out[0] & 0xff0000) >> 16) as u8,
            2 => ((out[0] & 0xff00) >> 8) as u8,
            1 => {
                j = 1;
                (out[0] & 0xff) as u8
            }
            _ => {
                i = 0;
                bin[0]
            }
        };

        while j < out.len() {
            bin[i] = ((out[j] >> 0x18) & 0xff) as u8;
            bin[i + 1] = ((out[j] >> 0x10) & 0xff) as u8;
            bin[i + 2] = ((out[j] >> 8) & 0xff) as u8;
            bin[i + 3] = ((out[j] >> 0) & 0xff) as u8;
            i += 4;
            j += 1;
        }

        let leading_zeros = bin.iter().take_while(|&&x| x == 0).count();
        let new_str = String::from_utf8(bin[leading_zeros - zcount..].to_vec());
        match new_str {
            Ok(res) => Ok(res),
            Err(_) => Err(DecodeError::Invalid),
        }
    }
}
