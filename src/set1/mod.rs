use std::{env::consts::ARCH, slice::Chunks};


mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;
mod challenge6;

#[allow(dead_code)]
fn byte_hex_to_raw(digit: u8) -> u8
{
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => (digit - b'a') + 10,
        _ => panic!("Invalid Hex character")
    }
}

// How do i take a nibble here........hmmmm I may have to make a new type
#[allow(dead_code)]
fn byte_raw_to_hex(digit: u8) -> [u8; 2] {
    // matching the higher bits and the lower bits (nibbles)
    let (ah, al) = (digit >> 4, digit & 0x0f);
    let h = match ah {
        0..=9 => b'0' + ah,
        10..=15 => b'a' + (ah - 10),
        _ => panic!("a single hex char cannot represent digit > 15, invalid nibble ({digit})")
    };
    let l = match al {
        0..=9 => b'0' + al,
        10..=15 => b'a' + (al - 10),
        _ => panic!("a single hex char cannot represent digit > 15, invalid nibble ({digit})")
    };

    [h, l]

}


#[allow(dead_code)]
pub fn decode_hex_bad(s: &str) -> impl Iterator<Item = u8> + '_ {
    s.bytes().map(byte_hex_to_raw)
}

pub fn encode_hex(s: Vec<u8>) -> String {
    let utf8_bytes: Vec<u8> = s.into_iter().flat_map(byte_raw_to_hex).collect();
    // should be safe to use this here, coz only valid hex chars are returned from byte_raw_to_hex
    unsafe {
        String::from_utf8_unchecked(utf8_bytes)
    }
}

pub fn decode_hex(s: &str) -> Vec<u8>  {
    let chunked = s.as_bytes().chunks_exact(2);
    if chunked.remainder().len() != 0 {
        panic!("Why is there an extra nibble here lmao, hexstring len % 2 != 0");
    }

    chunked.into_iter().map(|chunk| {
        if let [ah, al] = *chunk {
            let ah = byte_hex_to_raw(ah);
            let al = byte_hex_to_raw(al);
            // ih = i higher order
            // ih = i lower order
            ah << 4 | al
        } else {
            unreachable!("no nibbles ???");
        }

    }).collect()
}


fn byte_b64_to_raw(byte: u8) -> u8 {
    match byte {
        b'A'..=b'Z' => 0 + byte - b'A',
        b'a'..=b'z' => 26 + byte - b'a',
        b'0'..=b'9' => 52 + byte - b'0',
        b'+' => 62,
        b'/' => 63,
        b'=' | _ => panic!("byte outside std b64 value range {byte}")
    }
}

#[allow(dead_code)]
pub fn decode_b64(s: &str) -> Vec<u8> {
    assert!(s.len() % 4 == 0, "b64 input len should be multiple of 4");
    let mut decoded : Vec<u8> = Vec::with_capacity(s.len());

    let mut chunked = s.as_bytes().chunks_exact(4);
    let init = chunked.len() - 1;

    for chunk in chunked.by_ref().take(init) {
        // decode bytes and then 
        let chunk: [u8; 4]= chunk.try_into().unwrap();
        let a = byte_b64_to_raw(chunk[0]);
        let b = byte_b64_to_raw(chunk[1]);
        let c = byte_b64_to_raw(chunk[2]);
        let d = byte_b64_to_raw(chunk[3]);

        decoded.push(a << 2 | b >> 4);
        decoded.push(b << 4 | c >> 2);
        decoded.push(c << 6 | d);

    }

    if let Some(last_chunk) = chunked.next() {
        match last_chunk {
            // this match gonna be expensive :dard:
            // we can check if it ends with b'=' before hand and take the last 4 bytes out.
            [_, _, b'=', b'='] => {
                let a = byte_b64_to_raw(last_chunk[0]);
                let b = byte_b64_to_raw(last_chunk[1]);
                decoded.push(a << 2 | b >> 4);
            },
            [_, _, _, b'=']  => {
                let a = byte_b64_to_raw(last_chunk[0]);
                let b = byte_b64_to_raw(last_chunk[1]);
                let c = byte_b64_to_raw(last_chunk[2]);
                decoded.push(a << 2 | b >> 4);
                decoded.push(b << 4 | c >> 2);
            },
            _ => {
                let a = byte_b64_to_raw(last_chunk[0]);
                let b = byte_b64_to_raw(last_chunk[1]);
                let c = byte_b64_to_raw(last_chunk[2]);
                let d = byte_b64_to_raw(last_chunk[3]);

                decoded.push(a << 2 | b >> 4);
                decoded.push(b << 4 | c >> 2);
                decoded.push(c << 6 | d);
            }
        }

    }
    else {
        unreachable!("last time toh hga hi");
    }

    decoded
}