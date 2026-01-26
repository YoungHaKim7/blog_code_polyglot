fn by_moving() {
    let 안녕 = "안녕 ".to_string();
    let 반갑습니다 = "반갑습니다".to_string();

    // Concatenate strings using the '+' operator
    // 안되는 코드 되게 만들ㅇㅓ 보자
    let hello_world = 안녕 + &반갑습니다;
    println!("{}", hello_world);
}

fn by_cloning() {
    let 안녕 = "안녕 ".to_string();
    let 반갑습니다 = "반갑습니다";

    // Concatenate strings using the '+' operator
    let hello_world = 안녕.clone() + 반갑습니다;
    println!("{}", hello_world);
}

fn by_mutating() {
    let mut 안녕 = "안녕 ".to_string();
    let 반갑습니다 = "반갑습니다";

    // Concatenate strings using the '+' operator
    안녕.push_str(&반갑습니다);
    println!("{}", 안녕);
}

fn main() {
    by_moving();
    by_cloning();
    by_mutating();
}
