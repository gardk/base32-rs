use crate::{DecodeError, Encoding, CROCKFORD, EXTENDED_HEX, STANDARD, ZBASE32};

#[test]
fn fuzz() -> Result<(), DecodeError> {
    fn fuzz(config: Encoding) -> Result<(), DecodeError> {
        use rand::{distributions::Uniform, Rng};

        let mut rng = rand::thread_rng();

        let data_dist = Uniform::new_inclusive(u8::MIN, u8::MAX);

        let short_dist = Uniform::new_inclusive(1, 24);
        let medium_dist = Uniform::new_inclusive(64, 256);
        let long_dist = Uniform::new_inclusive(1024, 8192);

        for _ in 0..3 {
            let short: Vec<u8> = rng
                .sample_iter(&data_dist)
                .take(rng.sample(short_dist))
                .collect();
            let medium: Vec<u8> = rng
                .sample_iter(&data_dist)
                .take(rng.sample(medium_dist))
                .collect();
            let long: Vec<u8> = rng
                .sample_iter(&data_dist)
                .take(rng.sample(long_dist))
                .collect();

            let encoded = config.encode(&short);
            let decoded = config.decode(encoded)?;

            assert_eq!(decoded, short);

            let encoded = config.encode(&medium);
            let decoded = config.decode(encoded)?;

            assert_eq!(decoded, medium);

            let encoded = config.encode(&long);
            let decoded = config.decode(encoded)?;

            assert_eq!(decoded, long);
        }

        Ok(())
    }

    fuzz(STANDARD)?;
    fuzz(EXTENDED_HEX)?;
    fuzz(CROCKFORD)?;
    fuzz(ZBASE32)
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
