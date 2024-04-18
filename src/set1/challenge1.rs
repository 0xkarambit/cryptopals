
fn byte_hex_to_raw(digit: u8) -> u8
{
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => (digit - b'a') + 10,
        _ => panic!("Invalid Hex character")
    }
}

const PADDING: u8 = 63;

fn byte_raw_to_b64(digit: u8) -> u8 {
    match digit {
        0..=25 => b'A' + digit,
        26..=51 => b'a' + (digit - 26),
        52..=61 => b'0' + (digit - 52),
        62 => b'+',
        63 => b'/',
        _ => panic!("`{digit}` out of bound for Base64 convertion")
    }
}

// https://en.wikipedia.org/wiki/Base64
// how to convert hex to b64
// 0000 0000 (b2)
// in hex every 2 digits represent a full byte ie 1 digit is a nibble
//      base16 => max val = 0xf => 15 | b2(1111)
// b64 -> 1 digit = 6 bits
// b64 max value = / => 63 | b2(111111)  
// Why is there padding i forgot
// LCM or 4 and 6 => 2 * 2 * 3 => 12
//      therefore we can take (12/4) ie 3 hex digits and convert them into 2 b64 characters
// B64 doesnt align with the common bytes (8bits) we use everywhere so sometimes we need padding (=)
// 8 and 6 lcm => 2 * 2 * 2 * 3 => 24
// 24 bits = 3 Bytes = 4 b64 characters
// so if we convert 2 bytes to b64 we need to add a padding character , if we convert 1 byte we need to add 2 padding characters
// hmmm i think its better to take 24 bits ie 6 hex chars now 
//  so 24 / 6 = 4,   so we need to transform a `[i8; 4]` or in this case `[i4; 6]` into `buffer: [i6; 4]`

// better to perform operations in bigger chunks ig...
// what if we could select the chunk size based on the size of the input... by reduction


// struct raw { }
fn b64_to_string(bytes: Vec<u8>) -> String {
    String::from_utf8(
        bytes.into_iter()
                  .map(byte_raw_to_b64)
                  .collect::<Vec<_>>()
    ).unwrap()
}

#[allow(dead_code)]
fn hex_to_b64(s: &str) -> String {
    let raw:Vec<_> = s.bytes().map(byte_hex_to_raw).collect();

    let chunked = raw.chunks_exact(6);
    let tail = chunked.remainder();

    let mut result = "".to_string();
    for chunk in chunked {
        // chunk here is a [u8; 6] $ here only the LSB are useful to us ... well the others are 0 so it doesnt really matter
        // we are in big endian land here so
        // 6 hex -> 4 b64
        let b1 = (chunk[0] << 2) | chunk[1] >> 2;
        let b2 = ((chunk[1] & 0b11) << 4) | chunk[2];
        let b3 = (chunk[3] << 2) | chunk[4] >> 2;
        let b4 = ((chunk[4] & 0b11) << 4) | chunk[5];


        result.push_str(&b64_to_string(vec![b1, b2, b3, b4]));
        dbg!(&result);
    }

    // handle the tail as well. lmao (can we pattern match tail)
    let seq_with_padding: Vec<u8> = match *tail {
        [] => { vec![] },
        [_a] => {
            // Curious case of a single nibble lmao, maybe we can mark 
            // `fd df fd d` 
            // ig when converting the above to binary we would just mark it as `fddffdd0` or is it illegal to do that
            // https://base64.guru/converter/encode/hex Errors out on this.......mmm
            // I will panic as well then
            panic!("Invalid length of hex (doesnt align with bytes)");
            // vec![a, 0, PADDING, PADDING]
        },
        [a, b] => {
            vec![(a << 2) | (b >> 2), b & 0b11, PADDING, PADDING]
        },
        [a, b, c] => {
            vec![
                (a<< 2) | (b >> 2),
                ((b & 0b11) << 4) | c,
                PADDING,
                PADDING
            ]
        },
        [a, b, c, d] => {
            vec![
                (a<< 2) | (b >> 2),
                ((b & 0b11) << 4) | c,
                d << 2,
                PADDING
            ]
        },
        [a, b, c, d, e] => {
            // No padding is required (last char | 4th char) = (e & 0b11) << 4
            vec![
                (a<< 2) | (b >> 2),
                ((b & 0b11) << 4) | c,
                d << 2 | e >> 2,
                PADDING
            ]
        },
        _ => unreachable!("We have chunks of 6, so we can never get a len >= 6, {tail:?}")
    };

    result.push_str(&String::from_utf8(seq_with_padding.into_iter().map(byte_raw_to_b64).collect::<Vec<_>>()).unwrap());

    result

}


#[test]
fn test_challenge_1() {
    // this is not working -> "206b696"
    assert_eq!(
        hex_to_b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}

#[ignore = "dev"]
#[test]
fn test_hex_digit() {
    for (h, d) in (b'0'..=b'9').chain(b'a'..=b'f').zip(0..=15) {
        assert_eq!(byte_hex_to_raw(h), d);
    }
}