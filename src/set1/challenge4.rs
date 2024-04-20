use std::{cmp::Ordering, collections::HashMap, fmt::Display, iter, str::Lines};

use crate::set1::{challenge3::{analyse, score}, decode_hex};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Result {
    error: f64,
    key: u8,
    plaintext: String,
    cipher: String
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Result { error, key, plaintext, cipher } = self;
        f.write_str(&format!("Result {{ error: {error}, key: {key},\nplaintext: {plaintext},\ncipher: {cipher} }}"))
    }
}

// Solution Starts

// todo: I want to store the reference_score here globally

static mut BOOK_FREQMAP: Option<HashMap<u8, f64>> = None;

#[allow(dead_code)]
fn solve(lines: Lines<'_>) -> String {

    let mut buffer = vec![];
    for cipher in lines {
        println!("{cipher}");
        // xor this line with all bytes and take the top most ranking and put it in 
        // memory usage isnt high, maybe we can just put all the `xored` lines in a Vec
        // and rank the entire vector

        let cipher_bytes = decode_hex(cipher);

        (1..b'~').for_each(|key| {
            let plaintext = iter::repeat(key)
                .zip(cipher_bytes.iter())
                .map(|(b,c)| b ^ c)
                .collect::<Vec<_>>();

            let letter_freq = analyse(&plaintext);
            unsafe {
                let error = score(&letter_freq, BOOK_FREQMAP.as_ref().unwrap());
                match String::from_utf8(plaintext) {
                    Ok(plaintext) => {
                        let r = Result {
                            error,
                            key,
                            plaintext,
                            cipher: cipher.to_string(),
                        };
                        buffer.push(r);
                    },
                    Err(_) => {}
                }
            }
        })
    };

    // now we need to rank all these potential_plaintexts
    buffer.sort_by(|a, b| {
        if a.error < b.error {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });


    buffer.remove(0).plaintext
}


#[cfg(test)]
mod test_s1c4 {
    use std::fs;

    use crate::set1::challenge3::analyse;

    use super::*;

    #[test]
    fn test_solve() {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = format!("{root}/assets/4.txt");
        let file_path = format!("{root}/assets/pg61.txt");

        // todo: Read this in a common variable for all tests to use..
        let book = fs::read_to_string(file_path).expect("Book not found !");
        let book_bytes: Vec<_> = book.as_bytes().into();
        let contents = fs::read_to_string(path).expect("file not found");
        unsafe { BOOK_FREQMAP = Some(analyse(&book_bytes)) };


        let sol = solve(contents.lines());

        assert_eq!(sol, "Now that the party is jumping\n");
    }
}