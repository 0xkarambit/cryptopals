use super::{decode_hex, encode_hex};



#[allow(dead_code)]
fn fixed_xor(seq1: &str, seq2: &str) -> String {
    
    let result : Vec<u8> = decode_hex(&seq1).zip(decode_hex(&seq2))
                                .map(|(b1, b2)| b1 ^ b2)
                                .collect();
    encode_hex(result)
}


#[test]
fn test_fixed_xor() {
    assert_eq!(
        fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),
        "746865206b696420646f6e277420706c6179"
    )
}