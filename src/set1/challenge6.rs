
#[allow(dead_code)]
pub fn hamming_dist(a: &[u8], b: &[u8]) -> usize {
    // do both len need to be same ??
    // if a.len() == b.len() { return Err }
    a.into_iter().zip(b.into_iter())
    .map(|(&a, &b)| {
        // we need to calc binary hamming dist... no of bits that dont match..
        (a ^ b).count_ones()
    }).sum::<u32>() as usize
}


#[cfg(test)]
mod test_s1c6 {
    use std::fs;

    use crate::set1::decode_b64;

    use super::*;

    #[ignore = "dev"]
    #[test]
    fn test_b64_decode() {
        let res = decode_b64("VGhpcyBpcyBzb21lIHN0cmluZyB3aXRoIHBhZGRpbmc=");
        let decoded = String::from_utf8(res).unwrap();
        assert_eq!(decoded, "This is some string with padding")
    }

    #[ignore = "dev"]
    #[test]
    fn test_hamming_dist() {
        assert_eq!(
            hamming_dist(b"this is a test", b"wokka wokka!!!"),
            37
        )
    }

    #[test]
    fn test_solve() {
        // read file and solve
        let root = env!("CARGO_MANIFEST_DIR");
        let file_path = format!("{root}/assets/6.txt");

        // todo: Read this in a common variable for all tests to use..
        let book = fs::read_to_string(file_path).expect("Book not found !");

        assert!(false);
    }
}