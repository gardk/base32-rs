use crate::{Encoding, CROCKFORD, EXTENDED_HEX, STANDARD, ZBASE32};

#[test]
fn sanity_check() {
    fn fuzz(config: Encoding) {
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
            let decoded = config.decode(encoded).unwrap();

            assert_eq!(decoded, short);

            let encoded = config.encode(&medium);
            let decoded = config.decode(encoded).unwrap();

            assert_eq!(decoded, medium);

            let encoded = config.encode(&long);
            let decoded = config.decode(encoded).unwrap();

            assert_eq!(decoded, long);
        }
    }

    fuzz(STANDARD);
    fuzz(EXTENDED_HEX);
    fuzz(CROCKFORD);
    fuzz(ZBASE32);
}

#[test]
fn rfc4648_test_vectors_encode() {
    let encoder = STANDARD;

    assert_eq!(encoder.encode(""), "");
    assert_eq!(encoder.encode("f"), "MY======");
    assert_eq!(encoder.encode("fo"), "MZXQ====");
    assert_eq!(encoder.encode("foo"), "MZXW6===");
    assert_eq!(encoder.encode("foob"), "MZXW6YQ=");
    assert_eq!(encoder.encode("fooba"), "MZXW6YTB");
    assert_eq!(encoder.encode("foobar"), "MZXW6YTBOI======");

    let encoder = EXTENDED_HEX;

    assert_eq!(encoder.encode(""), "");
    assert_eq!(encoder.encode("f"), "CO======");
    assert_eq!(encoder.encode("fo"), "CPNG====");
    assert_eq!(encoder.encode("foo"), "CPNMU===");
    assert_eq!(encoder.encode("foob"), "CPNMUOG=");
    assert_eq!(encoder.encode("fooba"), "CPNMUOJ1");
    assert_eq!(encoder.encode("foobar"), "CPNMUOJ1E8======");

    let encoder = CROCKFORD;

    assert_eq!(encoder.encode(""), "");
    assert_eq!(encoder.encode("f"), "CR");
    assert_eq!(encoder.encode("fo"), "CSQG");
    assert_eq!(encoder.encode("foo"), "CSQPY");
    assert_eq!(encoder.encode("foob"), "CSQPYRG");
    assert_eq!(encoder.encode("fooba"), "CSQPYRK1");
    assert_eq!(encoder.encode("foobar"), "CSQPYRK1E8");

    let encoder = ZBASE32;

    assert_eq!(encoder.encode(""), "");
    assert_eq!(encoder.encode("f"), "ca");
    assert_eq!(encoder.encode("fo"), "c3zo");
    assert_eq!(encoder.encode("foo"), "c3zs6");
    assert_eq!(encoder.encode("foob"), "c3zs6ao");
    assert_eq!(encoder.encode("fooba"), "c3zs6aub");
    assert_eq!(encoder.encode("foobar"), "c3zs6aubqe");
}

#[test]
fn rfc4648_test_vectors_decode() {
    let decoder = STANDARD;

    assert_eq!(decoder.decode("").unwrap(), b"");
    assert_eq!(decoder.decode("MY======").unwrap(), b"f");
    assert_eq!(decoder.decode("MZXQ====").unwrap(), b"fo");
    assert_eq!(decoder.decode("MZXW6===").unwrap(), b"foo");
    assert_eq!(decoder.decode("MZXW6YQ=").unwrap(), b"foob");
    assert_eq!(decoder.decode("MZXW6YTB").unwrap(), b"fooba");
    assert_eq!(decoder.decode("MZXW6YTBOI======").unwrap(), b"foobar");

    let decoder = EXTENDED_HEX;

    assert_eq!(decoder.decode("").unwrap(), b"");
    assert_eq!(decoder.decode("CO======").unwrap(), b"f");
    assert_eq!(decoder.decode("CPNG====").unwrap(), b"fo");
    assert_eq!(decoder.decode("CPNMU===").unwrap(), b"foo");
    assert_eq!(decoder.decode("CPNMUOG=").unwrap(), b"foob");
    assert_eq!(decoder.decode("CPNMUOJ1").unwrap(), b"fooba");
    assert_eq!(decoder.decode("CPNMUOJ1E8======").unwrap(), b"foobar");

    let decoder = CROCKFORD;

    assert_eq!(decoder.decode("").unwrap(), b"");
    assert_eq!(decoder.decode("CR").unwrap(), b"f");
    assert_eq!(decoder.decode("CSQG").unwrap(), b"fo");
    assert_eq!(decoder.decode("CSQPY").unwrap(), b"foo");
    assert_eq!(decoder.decode("CSQPYRG").unwrap(), b"foob");
    assert_eq!(decoder.decode("CSQPYRK1").unwrap(), b"fooba");
    assert_eq!(decoder.decode("CSQPYRK1E8").unwrap(), b"foobar");

    let decoder = ZBASE32;

    assert_eq!(decoder.decode("").unwrap(), b"");
    assert_eq!(decoder.decode("ca").unwrap(), b"f");
    assert_eq!(decoder.decode("c3zo").unwrap(), b"fo");
    assert_eq!(decoder.decode("c3zs6").unwrap(), b"foo");
    assert_eq!(decoder.decode("c3zs6ao").unwrap(), b"foob");
    assert_eq!(decoder.decode("c3zs6aub").unwrap(), b"fooba");
    assert_eq!(decoder.decode("c3zs6aubqe").unwrap(), b"foobar");
}
