pub mod base58;

#[cfg(test)]
mod tests {
    use crate::base58::{Decoder, Encoder};

    #[test]
    fn encode_decode() {
        assert_eq!("ZiCa", "abc".encode_to_base58());
        assert_eq!(
            "我爱你iloveu",
            "7T5VrPqoBr9DeUXiUr2Fn".decode_from_base58().unwrap()
        );
    }
}
