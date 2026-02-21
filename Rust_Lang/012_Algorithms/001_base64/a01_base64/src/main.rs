use base64::{Engine as _, engine::general_purpose};

fn base64_encode(input: &[u8]) -> String {
    // URL-safe style alphabet similar to your Zig example:
    // "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+"
    // Note: Standard Rust URL_SAFE uses '-' and '_'
    // Your Zig uses '-' and '+', which is slightly custom.
    // We'll use STANDARD for closest match to '+'.

    general_purpose::STANDARD.encode(input)
}

fn main() {
    let result = base64_encode(b"h");
    println!("h __result : {}", result);

    let result02 = base64_encode(b"hello");
    println!("hello ___result02 : {}", result02);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(base64_encode(b"h"), "aA==");
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
    }

    #[test]
    fn fuzz_example() {
        for i in 0..255u8 {
            let input = [i];
            let encoded = base64_encode(&input);
            let decoded = general_purpose::STANDARD
                .decode(encoded)
                .expect("decode failed");
            assert_eq!(decoded, input);
        }
    }
}
