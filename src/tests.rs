use criterion::black_box;
use proptest::prelude::*;

use crate::{CROCKFORD, EXTENDED_HEX, STANDARD, ZBASE32};

proptest! {
    #[test]
    fn encode_doesnt_crash(s in "\\PC*") {
        let _ = black_box(STANDARD.encode(&s));
        let _ = black_box(EXTENDED_HEX.encode(&s));
        let _ = black_box(CROCKFORD.encode(&s));
        let _ = black_box(ZBASE32.encode(&s));
    }

    #[test]
    fn decode_doesnt_crash(s in "\\PC*") {
        let _ = black_box(STANDARD.decode(&s));
        let _ = black_box(EXTENDED_HEX.decode(&s));
        let _ = black_box(CROCKFORD.decode(&s));
        let _ = black_box(ZBASE32.decode(&s));
    }

    #[test]
    fn decode_standard_is_ok(s in "(?:[A-Z2-7]{8})*(?:[A-Z2-7]{2}={6}|[A-Z2-7]{4}={4}|[A-Z2-7]{5}={3}|[A-Z2-7]{7}=)?") {
        assert!(STANDARD.decode(s).is_ok());
    }

    #[test]
    fn decode_extended_hex_is_ok(s in "(?:[0-9A-V]{8})*(?:[0-9A-V]{2}={6}|[0-9A-V]{4}={4}|[0-9A-V]{5}={3}|[0-9A-V]{7}=)?") {
        assert!(EXTENDED_HEX.decode(s).is_ok());
    }

    #[test]
    fn decode_crockford_is_ok(s in "(?:[0-9A-HJKMNP-TV-Z]{8})*(?:[0-9A-HJKMNP-TV-Z]{2}|[0-9A-HJKMNP-TV-Z]{4}|[0-9A-HJKMNP-TV-Z]{5}|[0-9A-HJKMNP-TV-Z]{7})?") {
        assert!(CROCKFORD.decode(s).is_ok());
    }

    #[test]
    fn decode_zbase32_is_ok(s in "(?:[ybndrfg8ejkmcpqxot1uwisza345h769]{8})*(?:[ybndrfg8ejkmcpqxot1uwisza345h769]{2}|[ybndrfg8ejkmcpqxot1uwisza345h769]{4}|[ybndrfg8ejkmcpqxot1uwisza345h769]{5}|[ybndrfg8ejkmcpqxot1uwisza345h769]{7})?") {
        assert!(ZBASE32.decode(s).is_ok());
    }
}

#[test]
fn rfc4648_test_vectors_encode() {
    assert_eq!(STANDARD.encode(""), "");
    assert_eq!(STANDARD.encode("f"), "MY======");
    assert_eq!(STANDARD.encode("fo"), "MZXQ====");
    assert_eq!(STANDARD.encode("foo"), "MZXW6===");
    assert_eq!(STANDARD.encode("foob"), "MZXW6YQ=");
    assert_eq!(STANDARD.encode("fooba"), "MZXW6YTB");
    assert_eq!(STANDARD.encode("foobar"), "MZXW6YTBOI======");

    assert_eq!(EXTENDED_HEX.encode(""), "");
    assert_eq!(EXTENDED_HEX.encode("f"), "CO======");
    assert_eq!(EXTENDED_HEX.encode("fo"), "CPNG====");
    assert_eq!(EXTENDED_HEX.encode("foo"), "CPNMU===");
    assert_eq!(EXTENDED_HEX.encode("foob"), "CPNMUOG=");
    assert_eq!(EXTENDED_HEX.encode("fooba"), "CPNMUOJ1");
    assert_eq!(EXTENDED_HEX.encode("foobar"), "CPNMUOJ1E8======");

    assert_eq!(CROCKFORD.encode(""), "");
    assert_eq!(CROCKFORD.encode("f"), "CR");
    assert_eq!(CROCKFORD.encode("fo"), "CSQG");
    assert_eq!(CROCKFORD.encode("foo"), "CSQPY");
    assert_eq!(CROCKFORD.encode("foob"), "CSQPYRG");
    assert_eq!(CROCKFORD.encode("fooba"), "CSQPYRK1");
    assert_eq!(CROCKFORD.encode("foobar"), "CSQPYRK1E8");

    assert_eq!(ZBASE32.encode(""), "");
    assert_eq!(ZBASE32.encode("f"), "ca");
    assert_eq!(ZBASE32.encode("fo"), "c3zo");
    assert_eq!(ZBASE32.encode("foo"), "c3zs6");
    assert_eq!(ZBASE32.encode("foob"), "c3zs6ao");
    assert_eq!(ZBASE32.encode("fooba"), "c3zs6aub");
    assert_eq!(ZBASE32.encode("foobar"), "c3zs6aubqe");
}

#[test]
fn rfc4648_test_vectors_decode() -> Result<(), crate::DecodeError> {
    assert_eq!(STANDARD.decode("")?, b"");
    assert_eq!(STANDARD.decode("MY======")?, b"f");
    assert_eq!(STANDARD.decode("MZXQ====")?, b"fo");
    assert_eq!(STANDARD.decode("MZXW6===")?, b"foo");
    assert_eq!(STANDARD.decode("MZXW6YQ=")?, b"foob");
    assert_eq!(STANDARD.decode("MZXW6YTB")?, b"fooba");
    assert_eq!(STANDARD.decode("MZXW6YTBOI======")?, b"foobar");

    assert_eq!(EXTENDED_HEX.decode("")?, b"");
    assert_eq!(EXTENDED_HEX.decode("CO======")?, b"f");
    assert_eq!(EXTENDED_HEX.decode("CPNG====")?, b"fo");
    assert_eq!(EXTENDED_HEX.decode("CPNMU===")?, b"foo");
    assert_eq!(EXTENDED_HEX.decode("CPNMUOG=")?, b"foob");
    assert_eq!(EXTENDED_HEX.decode("CPNMUOJ1")?, b"fooba");
    assert_eq!(EXTENDED_HEX.decode("CPNMUOJ1E8======")?, b"foobar");

    assert_eq!(CROCKFORD.decode("")?, b"");
    assert_eq!(CROCKFORD.decode("CR")?, b"f");
    assert_eq!(CROCKFORD.decode("CSQG")?, b"fo");
    assert_eq!(CROCKFORD.decode("CSQPY")?, b"foo");
    assert_eq!(CROCKFORD.decode("CSQPYRG")?, b"foob");
    assert_eq!(CROCKFORD.decode("CSQPYRK1")?, b"fooba");
    assert_eq!(CROCKFORD.decode("CSQPYRK1E8")?, b"foobar");

    assert_eq!(ZBASE32.decode("")?, b"");
    assert_eq!(ZBASE32.decode("ca")?, b"f");
    assert_eq!(ZBASE32.decode("c3zo")?, b"fo");
    assert_eq!(ZBASE32.decode("c3zs6")?, b"foo");
    assert_eq!(ZBASE32.decode("c3zs6ao")?, b"foob");
    assert_eq!(ZBASE32.decode("c3zs6aub")?, b"fooba");
    assert_eq!(ZBASE32.decode("c3zs6aubqe")?, b"foobar");

    Ok(())
}

#[test]
fn custom_padding_encode() {
    let original = STANDARD.encode("foobar");

    assert_eq!(
        original.trim_end_matches('='),
        STANDARD.with_padding(None).encode("foobar")
    );
    assert_eq!(
        original.replace('=', "+"),
        STANDARD.with_padding(Some(b'+')).encode("foobar"),
    );
}

#[test]
fn custom_padding_decode() {
    let original = STANDARD.encode("foobar");

    assert!(STANDARD
        .with_padding(None)
        .decode(original.trim_end_matches('='))
        .is_ok());
    assert!(STANDARD
        .with_padding(Some(b'+'))
        .decode(original.replace('=', "+"))
        .is_ok());
}
