use super::encode_hex;


#[allow(dead_code)]
fn solve(plaintext: &str, key: &str) -> String {

    let result: Vec<_> = plaintext.bytes().zip(key.bytes().cycle())
                    .map(|(p, k)| p ^ k)
                    .collect();

    encode_hex(result)
}

#[cfg(test)]
mod test_s1c5 {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let key = "ICE";
        let exp_result = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let res = solve(input, key);

        dbg!(&res);

        assert_eq!(res, exp_result);
    }
}