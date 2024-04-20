
// If it has been xored against a single byte then it would be better to brute force

// This challenge appears to require me to do some frequency analysis....
// should be interesting

// I found this https://www.gutenberg.org/cache/epub/61/pg61.txt
// too smol imo
// This looks appropriately sized https://www.gutenberg.org/cache/epub/52811/pg52811.txt



// we are working with bytes here, so we will only do freq analysis over letters.. not words... although that might be working in a similar way
// There must be some very advanced gpt like algorithms for freq_analysis


use std::{cmp::Ordering, collections::HashMap, fmt::Display, fs, iter::repeat};
use Ordering::{Less, Greater};

use super::decode_hex;

type FreqCountMap = HashMap<u8, u64>;
pub type FreqMap = HashMap<u8, f64>;

#[allow(dead_code)]
pub fn analyse(s: &Vec<u8>) -> FreqMap {
    let mut map : FreqCountMap = HashMap::new();

    // Only count ascii letter
    let letters : Vec<_> = (u8::MIN..b'~').collect();
    let mut total_letters = 0;

    for &byte in s.iter().filter(|b| letters.contains(b)) {
        map.entry(byte)
            .and_modify(|f| *f += 1)
            .or_insert(1);
        total_letters += 1;
    }

    // character_occurence = char_freq/total_chars_freq
    let mut freq_map: FreqMap = HashMap::new();
    for (key, freq) in map.into_iter() {
        freq_map.insert(key, freq as f64 / total_letters as f64);
    }

    freq_map
}

#[allow(dead_code)]
pub fn score(cipher_m: &FreqMap, reference_m: &FreqMap) -> f64 {
    cipher_m
        .into_iter()
        .map(|(key, freq)| {
            match reference_m.get(key) {
                Some(&ref_freq) => f64::abs(ref_freq - freq),
                None => 0.1_f64
            }
        }).sum()
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Result {
    error: f64,
    key: u8,
    plaintext: String
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Result { error, key, plaintext } = self;
        f.write_str(&format!("Result {{ error: {error}, key: {key}, plaintext: {plaintext}  }}"))
    }
}


#[allow(dead_code)]
fn single_byte_xor(cipher: &str) -> Vec<Result>  {
    // read file and perform analysis
    let root = env!("CARGO_MANIFEST_DIR");
    let file_path = format!("{root}/assets/pg61.txt");
    let book = fs::read_to_string(file_path).expect("Book not found !");
    let book_vec: Vec<_> = book.as_bytes().into();

    let book_freqmap = analyse(&book_vec);
    let cipher: Vec<_> = decode_hex(cipher);

    let mut results: Vec<Result> = vec![];

    // xor with 0 changes nothing, do we still wanna keep it here lol
    for key in u8::MIN+1..=u8::MAX {
        let full_key = std::iter::repeat(key);

        // xor `ciper` with every byte 
        let xored: Vec<_> = cipher.iter()
                                    .zip(full_key)
                                    .map(|(c, b)| c ^ b).collect();

        let c2 = xored.clone();
        let ascii = match String::from_utf8(c2) {
            Ok(s) => s,
            Err(_) => continue
        };

        let c_f = analyse(&xored);
        let err = score(&c_f, &book_freqmap);

        results.push(Result { error: err, key, plaintext: ascii });

    }

    results.sort_by(|a, b| {
        if a.error < b.error { Less } else { Greater }
    });
    // best_match
    results

}


#[allow(dead_code)]
fn brute_force(cipher: &str) -> Vec<(String, u8)> {

    let cipher: Vec<_> = decode_hex(cipher);
    let mut results: Vec<(String, u8)> = Vec::new();

    for key in 0..127_u8 {
        let full_key= repeat(key);
        let xored: Vec<u8> = cipher.iter().zip(full_key).map(|(c, b)| c ^ b).collect();
        
        match String::from_utf8(xored) {
            Ok(string) => results.push((string, key)),
            Err(_) => todo!(),
        }
        
    }
    return results;
}



#[test]
fn test_freq_anal() {
    let res = single_byte_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    assert_eq!(res[0].plaintext, "Cooking MC's like a pound of bacon");
}