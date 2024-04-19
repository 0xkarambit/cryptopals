
// If it has been xored against a single byte then it would be better to brute force

// This challenge appears to require me to do some frequency analysis....
// should be interesting

// I found this https://www.gutenberg.org/cache/epub/61/pg61.txt
// too smol imo
// This looks appropriately sized https://www.gutenberg.org/cache/epub/52811/pg52811.txt



// we are working with bytes here, so we will only do freq analysis over letters.. not words... although that might be working in a similar way
// There must be some very advanced gpt like algorithms for freq_analysis


use std::{collections::HashMap, f64::NEG_INFINITY, fs, iter::repeat, string};

use super::{decode_hex, decode_hex_bad};

type FreqCountMap = HashMap<u8, u64>;
type FreqMap = HashMap<u8, f64>;

#[allow(dead_code)]
fn analyse(s: &mut impl Iterator<Item = u8>) -> FreqMap {
// fn analyse(s: &str) -> FreqMap {
    let mut map : FreqCountMap = HashMap::new();

    // let letters : Vec<_> = (b'0'..=b'z').chain(b'A'..=b'Z').chain([b' ', b'\''].into_iter()).collect();
    let letters : Vec<_> = (u8::MIN..b'~').collect();
    let mut total_letters = 0;

    // counting each character
    // is byte better than &byte here.... ?
    for byte in s {
        // Only count ascii letter
        if letters.contains(&byte) {
            map.entry(byte)
                .and_modify(|f| *f += 1)
                .or_insert(1);
            total_letters += 1;
        }
    }

    // finding rate of occurence
    // character_occurence = char_freq/total_chars_freq
    let mut freq_map: FreqMap = HashMap::new();
    for (key, freq) in map.into_iter() {
        freq_map.insert(key, freq as f64 / total_letters as f64);
    }

    freq_map
}

// I think we have to determine all the different way it can be xor'd and choose the highest ranking one using a score function
// SO how do i score if a text is valid english....
// hmmmm i guess i can compare the no of times letters exist there...
// Or do i use a set of words
// will try letter frequencies rn


#[allow(dead_code)]
fn score(cipher_m: &FreqMap, reference_m: &FreqMap) -> f64 {
    // compare them both and find the most matching char by freq, and returns error in matching as f64
    let mut error = 0_f64;
    for (key, freq) in cipher_m.into_iter() {
        match reference_m.get(key) {
            // we need to score by how much `freq` matches `req_freq`
            Some(&ref_freq) =>  error += f64::abs(ref_freq - freq),
            // If a key doesnt exist in ref at all but does in cipher its probably not the correct one
            // (NON STANDARD letters) so we should reduce the score... but by how much ?? 0.1 ??
            // Oh wait the FreqMap is only of ascii_letter utf8s so this is 
            // None => unreachable!("maji ?")
            None => {}
        }
    }
    error
}


#[allow(dead_code)]
fn single_byte_xor(cipher: &str) -> String {
    // read file and perform analysis
    let root = env!("CARGO_MANIFEST_DIR");
    let file_path = format!("{root}/assets/pg61.txt");
    let book = fs::read_to_string(file_path).expect("Book not found !");

    let book_freqmap = analyse(&mut book.bytes());
    // let cipher_freqmap = analyse(&cipher.bytes());

    let mut best_match: String = "".into();
    let mut min_error = f64::INFINITY;

    // xor with 0 changes nothing, do we still wanna keep it here lol
    for key in u8::MIN..=u8::MAX {
        let full_key = std::iter::repeat(key);
        // xor `ciper` with every byte 
        let mut candidate = cipher.bytes()
                                                            .zip(full_key)
                                                            .map(|(c, b)| c ^ b);
        let c2 = candidate.clone();

        let mut ascii: Option<String> = None;

        match String::from_utf8(c2.collect::<Vec<u8>>()) {
            Ok(str2) => ascii = Some(str2),
            // Err(_e) => {}
            Err(_e) => continue
        }

        let c_f = analyse(&mut candidate);
        let s = score(&c_f, &book_freqmap);

        let word = ascii.unwrap();
        if !word.contains("\n") {
            // println!("{word} got score {s}");
        }


        if s < min_error {
            min_error = s;
            best_match = word;
        }
        
    }
    // dbg!(min_error);

    best_match
    // compare each key in cipher_freqmap and find the key in book_freqmap that matches it frequency most closely...
    // We need a ratio here most likely

}


#[allow(dead_code)]
fn brute_force(cipher: &str) -> Vec<String> {
    // let cipher = cipher.as_bytes();
    let cipher: Vec<_> = decode_hex(cipher);
    // let len = cipher.len();

    let mut results: Vec<String> = Vec::new();

    for key in 0..127_u8 {
        // let full_key: Vec<_> = repeat(key).take(len).collect();
        let full_key= repeat(key);
        // xor both
        let xored: Vec<u8> = cipher.iter().zip(full_key).map(|(c, b)| c ^ b).collect();
        
        match String::from_utf8(xored) {
            Ok(string) => results.push(string),
            Err(_) => todo!(),
        }
        
    }
    return results;
}

#[test]
fn test_freq_anal() {
    // println!("{}", single_byte_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    // only outputs when the test fails

    let res = brute_force("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    for i in res {
        println!("{}", i);
    }

    assert!(false)
}