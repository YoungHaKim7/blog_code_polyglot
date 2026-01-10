fn main() {
    let my_str = "안녕하세요.";
    for char in my_str.chars() {
        let code_point = char as u32;
        println!("문자 '{}': U+{:04X}", char, code_point);
    }
    // UTF-8 인코딩
    let utf8_encoded = my_str.as_bytes();
    println!("UTF-8 인코딩 결과: {:?}", utf8_encoded);

    // UTF-16 인코딩
    let utf16_encoded: Vec<u16> = my_str.encode_utf16().collect();
    println!("UTF-16 인코딩 결과: {:?}", utf16_encoded);
    // UTF-32 인코딩
    let utf32_encoded: Vec<u32> = my_str.chars().map(|c| c as u32).collect();
    println!("UTF-32 인코딩 결과: {:?}", utf32_encoded);

    println!();
    let my_str02 = "테스트";
    println!("테스트 : {:?}", my_str02.as_bytes());
}
